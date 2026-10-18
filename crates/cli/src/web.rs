use std::collections::BTreeMap;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

use acvc_domain::{Error, Result};
use serde_json::{Map, Value, json};

use crate::args::Format;
use crate::present;
use crate::web_page::page;

pub fn listen(repo: Option<&Path>, bind: &str, format: Format) -> Result<()> {
    let listener =
        TcpListener::bind(bind).map_err(|err| Error::new(err.to_string()).ctx("bind", bind))?;
    let addr = listener
        .local_addr()
        .map_err(|err| Error::new(err.to_string()).ctx("op", "local_addr"))?;
    let (value, text) = present::listening(&addr.to_string());
    present::emit(format, value, text);
    let _ = std::io::stdout().flush();
    let repo = repo.map(Path::to_path_buf);
    for incoming in listener.incoming() {
        let stream = incoming.map_err(|err| Error::new(err.to_string()).ctx("op", "accept"))?;
        respond(&repo, stream)?;
    }
    Ok(())
}

fn respond(repo: &Option<PathBuf>, mut stream: TcpStream) -> Result<()> {
    let _ = stream.set_read_timeout(Some(Duration::from_secs(5)));
    let Some(text) = read_headers(&mut stream)? else {
        return Ok(());
    };
    let request = text.lines().next().unwrap_or("");
    let mut parts = request.split_whitespace();
    let method = parts.next().unwrap_or("");
    let target = parts.next().unwrap_or("/");
    if method != "GET" {
        return write_response(&mut stream, 405, "text/plain; charset=utf-8", "method");
    }
    let (path, query) = split_target(target)?;
    let built = route(repo, path, &query);
    let (status, content_type, body) = match built {
        Ok(parts) => parts,
        Err(err) => (422, "application/json", error_json(&err)),
    };
    write_response(&mut stream, status, content_type, &body)
}

fn read_headers(stream: &mut TcpStream) -> Result<Option<String>> {
    let mut buf = Vec::new();
    let mut chunk = [0u8; 1024];
    while !buf.windows(4).any(|window| window == b"\r\n\r\n") {
        let read = match stream.read(&mut chunk) {
            Ok(0) => return Ok(None),
            Ok(read) => read,
            Err(err) if err.kind() == std::io::ErrorKind::TimedOut => return Ok(None),
            Err(err) if err.kind() == std::io::ErrorKind::WouldBlock => return Ok(None),
            Err(err) => return Err(Error::new(err.to_string()).ctx("op", "http read")),
        };
        buf.extend_from_slice(&chunk[..read]);
        if buf.len() > 8192 {
            return Err(Error::new("http header is too large"));
        }
    }
    String::from_utf8(buf)
        .map(Some)
        .map_err(|_| Error::new("http header is not utf-8"))
}

fn route(
    repo: &Option<PathBuf>,
    path: &str,
    query: &BTreeMap<String, String>,
) -> Result<(u16, &'static str, String)> {
    match path {
        "/" => Ok((200, "text/html; charset=utf-8", page())),
        "/api/campaigns" => json_or_cli(repo, &["campaign", "list"]),
        "/api/frontier" => {
            let campaign = required(query, "campaign")?;
            json_or_cli(repo, &["frontier", "--campaign", &campaign])
        }
        "/api/why" => {
            let variant = required(query, "variant")?;
            json_or_cli(repo, &["why", &variant])
        }
        "/api/recall" => {
            let campaign = required(query, "campaign")?;
            let needle = required(query, "query")?;
            json_or_cli(
                repo,
                &["recall", "--campaign", &campaign, "--query", &needle],
            )
        }
        "/api/compare" => {
            let left = required(query, "left")?;
            let right = required(query, "right")?;
            json_or_cli(repo, &["compare", &left, &right])
        }
        "/api/context" => {
            let campaign = required(query, "campaign")?;
            json_or_cli(repo, &["context", "--campaign", &campaign])
        }
        "/api/file-diff" => {
            let left = required(query, "left")?;
            let right = required(query, "right")?;
            let path = required(query, "path")?;
            json_or_cli(repo, &["file-diff", &left, &right, "--path", &path])
        }
        _ => Ok((404, "text/plain; charset=utf-8", "not found".to_string())),
    }
}

fn error_json(err: &Error) -> String {
    let mut context = Map::new();
    for (key, value) in &err.context {
        context.insert(key.clone(), Value::String(value.clone()));
    }
    json!({"error": err.message, "context": context}).to_string()
}

fn json_or_cli(repo: &Option<PathBuf>, args: &[&str]) -> Result<(u16, &'static str, String)> {
    let (ok, stdout, stderr) = run_cli(repo, args)?;
    if ok {
        Ok((200, "application/json", stdout))
    } else {
        Ok((422, "application/json", stderr))
    }
}

fn run_cli(repo: &Option<PathBuf>, args: &[&str]) -> Result<(bool, String, String)> {
    let exe =
        std::env::current_exe().map_err(|err| Error::new(err.to_string()).ctx("op", "exe"))?;
    let mut command = Command::new(exe);
    if let Some(repo) = repo {
        command.arg("--repo").arg(repo);
    }
    let output = command
        .args(args)
        .output()
        .map_err(|err| Error::new(err.to_string()).ctx("op", "spawn"))?;
    Ok((
        output.status.success(),
        String::from_utf8_lossy(&output.stdout).into_owned(),
        String::from_utf8_lossy(&output.stderr).into_owned(),
    ))
}

fn required(query: &BTreeMap<String, String>, name: &str) -> Result<String> {
    query
        .get(name)
        .cloned()
        .filter(|value| !value.is_empty())
        .ok_or_else(|| Error::new("missing query").ctx("field", name))
}

fn split_target(target: &str) -> Result<(&str, BTreeMap<String, String>)> {
    let (path, raw) = target.split_once('?').unwrap_or((target, ""));
    let mut query = BTreeMap::new();
    if !raw.is_empty() {
        for pair in raw.split('&') {
            let (key, value) = pair.split_once('=').unwrap_or((pair, ""));
            query.insert(decode(key)?, decode(value)?);
        }
    }
    Ok((path, query))
}

fn decode(raw: &str) -> Result<String> {
    let bytes = raw.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        match bytes[index] {
            b'+' => {
                out.push(b' ');
                index += 1;
            }
            b'%' if index + 2 < bytes.len() => {
                let hex = std::str::from_utf8(&bytes[index + 1..index + 3])
                    .map_err(|_| Error::new("bad query escape"))?;
                let byte =
                    u8::from_str_radix(hex, 16).map_err(|_| Error::new("bad query escape"))?;
                out.push(byte);
                index += 3;
            }
            byte => {
                out.push(byte);
                index += 1;
            }
        }
    }
    String::from_utf8(out).map_err(|_| Error::new("query is not utf-8"))
}

fn write_response(
    stream: &mut TcpStream,
    status: u16,
    content_type: &str,
    body: &str,
) -> Result<()> {
    let reason = match status {
        200 => "OK",
        404 => "Not Found",
        405 => "Method Not Allowed",
        422 => "Unprocessable Entity",
        _ => "Error",
    };
    write!(
        stream,
        "HTTP/1.1 {status} {reason}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    )
    .and_then(|_| stream.flush())
    .map_err(|err| Error::new(err.to_string()).ctx("op", "http write"))?;
    let _ = stream.shutdown(Shutdown::Both);
    Ok(())
}
