use std::io::{BufRead, BufReader, Write};

use acvc_domain::{Error, Result};
use serde_json::{Value, json};

use crate::mcp_tools;

pub fn serve() -> Result<()> {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut stdout = std::io::stdout().lock();
    loop {
        let Some(message) = read_message(&mut reader)? else {
            return Ok(());
        };
        if let Some(response) = handle(message) {
            write_message(&mut stdout, &response)?;
        }
    }
}

fn handle(message: Value) -> Option<Value> {
    let id = message.get("id").filter(|id| !id.is_null()).cloned()?;
    let method = message.get("method").and_then(Value::as_str).unwrap_or("");
    let result = match method {
        "initialize" => Ok(json!({
            "protocolVersion": message
                .pointer("/params/protocolVersion")
                .and_then(Value::as_str)
                .unwrap_or("2024-11-05"),
            "capabilities": {"tools": {}},
            "serverInfo": {"name": "acvc", "version": env!("CARGO_PKG_VERSION")}
        })),
        "tools/list" => Ok(mcp_tools::list_tools()),
        "tools/call" => call(&message),
        _ => Err(Error::new("method not found").ctx("method", method)),
    };
    Some(match result {
        Ok(result) => json!({"jsonrpc": "2.0", "id": id, "result": result}),
        Err(err) => json!({
            "jsonrpc": "2.0",
            "id": id,
            "error": {"code": -32601, "message": err.to_string()}
        }),
    })
}

fn call(message: &Value) -> Result<Value> {
    let name = message
        .pointer("/params/name")
        .and_then(Value::as_str)
        .ok_or_else(|| Error::new("tool name is missing"))?;
    let arguments = message
        .pointer("/params/arguments")
        .cloned()
        .unwrap_or(Value::Object(serde_json::Map::new()));
    let (failed, text) = mcp_tools::call(name, &arguments)?;
    Ok(json!({
        "content": [{"type": "text", "text": text}],
        "isError": failed
    }))
}

fn read_message(reader: &mut impl BufRead) -> Result<Option<Value>> {
    let mut length = None;
    loop {
        let mut line = String::new();
        let read = reader
            .read_line(&mut line)
            .map_err(|err| Error::new(err.to_string()).ctx("op", "mcp read"))?;
        if read == 0 {
            return Ok(None);
        }
        if line == "\n" || line == "\r\n" {
            break;
        }
        if let Some(rest) = line.to_ascii_lowercase().strip_prefix("content-length:") {
            length = rest.trim().parse::<usize>().ok();
        }
    }
    let length = length.ok_or_else(|| Error::new("mcp frame has no content length"))?;
    if length > 8 * 1024 * 1024 {
        return Err(Error::new("mcp frame exceeds 8MB").ctx("len", length));
    }
    let mut body = vec![0u8; length];
    reader
        .read_exact(&mut body)
        .map_err(|err| Error::new(err.to_string()).ctx("op", "mcp body"))?;
    serde_json::from_slice(&body)
        .map(Some)
        .map_err(|err| Error::new(err.to_string()).ctx("op", "mcp json"))
}

fn write_message(writer: &mut impl Write, value: &Value) -> Result<()> {
    let body = serde_json::to_vec(value)
        .map_err(|err| Error::new(err.to_string()).ctx("op", "mcp json"))?;
    write!(writer, "Content-Length: {}\r\n\r\n", body.len())
        .and_then(|_| writer.write_all(&body))
        .and_then(|_| writer.flush())
        .map_err(|err| Error::new(err.to_string()).ctx("op", "mcp write"))
}
