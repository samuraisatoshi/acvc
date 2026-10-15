use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

use acvc_app::{App, ObjectBytes, TransferReport};
use acvc_domain::{Error, Hash, Result, Variant, missing_hashes};

use crate::args::Format;
use crate::present;
use crate::wire::{self, Body};

const TIMEOUT: Duration = Duration::from_secs(30);

pub fn listen(app: &App, bind: &str, format: Format) -> Result<()> {
    let listener =
        TcpListener::bind(bind).map_err(|err| Error::new(err.to_string()).ctx("bind", bind))?;
    let addr = listener
        .local_addr()
        .map_err(|err| Error::new(err.to_string()).ctx("op", "local_addr"))?;
    let (value, text) = present::listening(&addr.to_string());
    present::emit(format, value, text);
    let _ = std::io::stdout().flush();
    for incoming in listener.incoming() {
        let stream = incoming.map_err(|err| Error::new(err.to_string()).ctx("op", "accept"))?;
        let stream = prepare(stream)?;
        serve_conn(app, stream)?;
    }
    Ok(())
}

pub fn push_variant(app: &App, addr: &str, id: &Hash, campaign: &str) -> Result<TransferReport> {
    let mut io = connect(addr)?;
    let offer = app.offer_variant(id)?;
    let hashes: Vec<Hash> = offer.objects.iter().map(|object| object.hash).collect();
    let encoded = wire::encode_offer(&offer.variant, &offer.canonical, &hashes)?;
    wire::handshake(&mut io)?;
    wire::write_frame(&mut io, wire::PUSH, &wire::encode_name(campaign, &encoded)?)?;
    let need = wire::decode_hashes(&read_expect(&mut io, wire::NEED)?)?;
    refuse_unknown(&hashes, &need)?;
    let bodies = select(&offer.objects, &need)?;
    let payload_bytes = payload_len(&bodies);
    wire::write_frame(&mut io, wire::BODIES, &wire::encode_bodies(&bodies)?)?;
    let (got, graph) = wire::decode_done(&read_expect(&mut io, wire::DONE)?)?;
    if got != offer.variant {
        return Err(Error::new("remote variant id changed").ctx("variant", got.to_hex()));
    }
    Ok(report(offer.variant, &hashes, &need, payload_bytes, graph))
}

pub fn fetch_variant(app: &App, addr: &str, id: &Hash, campaign: &str) -> Result<TransferReport> {
    let mut io = connect(addr)?;
    wire::handshake(&mut io)?;
    wire::write_frame(&mut io, wire::FETCH, id.as_bytes())?;
    let (got, canonical, hashes) = wire::decode_offer(&read_expect(&mut io, wire::OFFER)?)?;
    if got != *id {
        return Err(Error::new("remote variant id changed").ctx("variant", got.to_hex()));
    }
    let variant = Variant::decode(&canonical)?;
    if variant.id()? != *id {
        return Err(
            Error::new("variant encoding does not match the requested id")
                .ctx("variant", id.to_hex()),
        );
    }
    let present = app.objects_present(&hashes)?;
    let missing = missing_hashes(&hashes, &present);
    wire::write_frame(&mut io, wire::NEED, &wire::encode_hashes(&missing)?)?;
    let bodies = wire::decode_bodies(&read_expect(&mut io, wire::BODIES)?)?;
    if !same_hashes(&bodies, &missing) {
        return Err(Error::new("body list does not match the request"));
    }
    let payload_bytes = payload_len(&bodies);
    let objects = to_objects(bodies);
    let accepted = app.accept_variant(campaign, &canonical, &objects)?;
    if accepted.variant != *id {
        return Err(
            Error::new("accepted variant id changed").ctx("variant", accepted.variant.to_hex())
        );
    }
    Ok(report(
        *id,
        &hashes,
        &missing,
        payload_bytes,
        accepted.graph,
    ))
}

fn serve_conn(app: &App, mut io: impl Read + Write) -> Result<()> {
    let result = session(app, &mut io);
    if let Err(err) = &result {
        let _ = wire::write_frame(&mut io, wire::FAIL, err.to_string().as_bytes());
    }
    result
}

fn session(app: &App, io: &mut (impl Read + Write)) -> Result<()> {
    wire::handshake(io)?;
    let frame = wire::read_frame(io)?;
    match frame.kind {
        wire::FETCH => fetch_server(app, io, &frame.body),
        wire::PUSH => push_server(app, io, &frame.body),
        other => Err(Error::new("unexpected frame").ctx("kind", other)),
    }
}

fn fetch_server(app: &App, io: &mut (impl Read + Write), body: &[u8]) -> Result<()> {
    let id = hash_of(body)?;
    let offer = app.offer_variant(&id)?;
    let hashes: Vec<Hash> = offer.objects.iter().map(|object| object.hash).collect();
    wire::write_frame(
        io,
        wire::OFFER,
        &wire::encode_offer(&offer.variant, &offer.canonical, &hashes)?,
    )?;
    let need = wire::decode_hashes(&read_expect(io, wire::NEED)?)?;
    refuse_unknown(&hashes, &need)?;
    let bodies = select(&offer.objects, &need)?;
    wire::write_frame(io, wire::BODIES, &wire::encode_bodies(&bodies)?)
}

fn push_server(app: &App, io: &mut (impl Read + Write), body: &[u8]) -> Result<()> {
    let (campaign, offer_bytes) = wire::decode_name(body)?;
    let (id, canonical, hashes) = wire::decode_offer(&offer_bytes)?;
    let variant = Variant::decode(&canonical)?;
    if variant.id()? != id {
        return Err(Error::new("variant id does not match").ctx("variant", id.to_hex()));
    }
    let present = app.objects_present(&hashes)?;
    let missing = missing_hashes(&hashes, &present);
    wire::write_frame(io, wire::NEED, &wire::encode_hashes(&missing)?)?;
    let bodies = wire::decode_bodies(&read_expect(io, wire::BODIES)?)?;
    if !same_hashes(&bodies, &missing) {
        return Err(Error::new("body list does not match the request"));
    }
    let accepted = app.accept_variant(&campaign, &canonical, &to_objects(bodies))?;
    if accepted.variant != id {
        return Err(
            Error::new("accepted variant id changed").ctx("variant", accepted.variant.to_hex())
        );
    }
    wire::write_frame(
        io,
        wire::DONE,
        &wire::encode_done(&accepted.variant, accepted.graph)?,
    )
}

fn select(objects: &[ObjectBytes], need: &[Hash]) -> Result<Vec<Body>> {
    let mut bodies = Vec::with_capacity(need.len());
    for hash in need {
        let object = objects
            .iter()
            .find(|object| object.hash == *hash)
            .ok_or_else(|| Error::new("object is not in the offer").ctx("hash", hash.to_hex()))?;
        bodies.push(Body {
            hash: object.hash,
            kind: object.kind,
            bytes: object.bytes.clone(),
        });
    }
    Ok(bodies)
}

fn to_objects(bodies: Vec<Body>) -> Vec<ObjectBytes> {
    bodies
        .into_iter()
        .map(|body| ObjectBytes {
            hash: body.hash,
            kind: body.kind,
            bytes: body.bytes,
        })
        .collect()
}

fn report(
    variant: Hash,
    offered: &[Hash],
    sent: &[Hash],
    payload_bytes: u64,
    graph: &str,
) -> TransferReport {
    let objects_skipped = offered.iter().filter(|hash| !sent.contains(hash)).count() as u64;
    TransferReport {
        variant,
        objects_sent: sent.len() as u64,
        objects_skipped,
        payload_bytes,
        graph: graph.to_string(),
    }
}

fn payload_len(bodies: &[Body]) -> u64 {
    bodies.iter().map(|body| body.bytes.len() as u64).sum()
}

fn same_hashes(bodies: &[Body], need: &[Hash]) -> bool {
    bodies.len() == need.len()
        && bodies
            .iter()
            .zip(need)
            .all(|(body, hash)| body.hash == *hash)
}

fn refuse_unknown(offered: &[Hash], need: &[Hash]) -> Result<()> {
    for hash in need {
        if !offered.contains(hash) {
            return Err(
                Error::new("peer asked for an object outside the offer").ctx("hash", hash.to_hex())
            );
        }
    }
    Ok(())
}

fn hash_of(bytes: &[u8]) -> Result<Hash> {
    if bytes.len() != Hash::LEN {
        return Err(Error::new("variant id must be 32 bytes").ctx("len", bytes.len()));
    }
    let mut raw = [0u8; Hash::LEN];
    raw.copy_from_slice(bytes);
    Ok(Hash::from_bytes(raw))
}

fn read_expect(io: &mut impl Read, kind: u8) -> Result<Vec<u8>> {
    let frame = wire::read_frame(io)?;
    if frame.kind == wire::FAIL {
        let message = String::from_utf8_lossy(&frame.body);
        return Err(Error::new(message.to_string()).ctx("op", "remote"));
    }
    wire::expect(frame, kind)
}

fn connect(addr: &str) -> Result<TcpStream> {
    let stream =
        TcpStream::connect(addr).map_err(|err| Error::new(err.to_string()).ctx("addr", addr))?;
    prepare(stream)
}

fn prepare(stream: TcpStream) -> Result<TcpStream> {
    stream
        .set_read_timeout(Some(TIMEOUT))
        .map_err(|err| Error::new(err.to_string()).ctx("op", "timeout"))?;
    stream
        .set_write_timeout(Some(TIMEOUT))
        .map_err(|err| Error::new(err.to_string()).ctx("op", "timeout"))?;
    stream
        .set_nodelay(true)
        .map_err(|err| Error::new(err.to_string()).ctx("op", "nodelay"))?;
    Ok(stream)
}
