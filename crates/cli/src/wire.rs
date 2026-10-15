use std::io::{Read, Write};

use acvc_domain::{Error, Hash, Result};

pub const MAGIC: &[u8; 8] = b"ACVCNET1";
pub const FETCH: u8 = 1;
pub const OFFER: u8 = 2;
pub const NEED: u8 = 3;
pub const BODIES: u8 = 4;
pub const DONE: u8 = 5;
pub const PUSH: u8 = 6;
pub const FAIL: u8 = 7;

const MAX_BYTES: u32 = 512 * 1024 * 1024;
const MAX_OBJECTS: u32 = 1_000_000;

pub struct Frame {
    pub kind: u8,
    pub body: Vec<u8>,
}

pub struct Body {
    pub hash: Hash,
    pub kind: u8,
    pub bytes: Vec<u8>,
}

pub fn handshake(io: &mut (impl Read + Write)) -> Result<()> {
    io.write_all(MAGIC)
        .map_err(|err| Error::new(err.to_string()).ctx("op", "write magic"))?;
    let mut magic = [0u8; 8];
    read_exact(io, &mut magic)?;
    if &magic != MAGIC {
        return Err(Error::new("bad remote magic"));
    }
    Ok(())
}

pub fn write_frame(io: &mut impl Write, kind: u8, body: &[u8]) -> Result<()> {
    let len = u32::try_from(body.len()).map_err(|_| Error::new("frame exceeds 4GB"))?;
    if len > MAX_BYTES {
        return Err(Error::new("frame exceeds 512MB").ctx("len", len));
    }
    io.write_all(&[kind])
        .and_then(|_| io.write_all(&len.to_le_bytes()))
        .and_then(|_| io.write_all(body))
        .and_then(|_| io.flush())
        .map_err(|err| Error::new(err.to_string()).ctx("op", "write frame"))
}

pub fn read_frame(io: &mut impl Read) -> Result<Frame> {
    let mut head = [0u8; 5];
    read_exact(io, &mut head)?;
    let len = u32::from_le_bytes(head[1..5].try_into().expect("4 bytes"));
    if len > MAX_BYTES {
        return Err(Error::new("frame exceeds 512MB").ctx("len", len));
    }
    let mut body = vec![0u8; len as usize];
    read_exact(io, &mut body)?;
    Ok(Frame {
        kind: head[0],
        body,
    })
}

pub fn expect(frame: Frame, kind: u8) -> Result<Vec<u8>> {
    if frame.kind != kind {
        return Err(Error::new("unexpected frame").ctx("kind", frame.kind));
    }
    Ok(frame.body)
}

pub struct Cursor<'a> {
    data: &'a [u8],
    at: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, at: 0 }
    }

    pub fn take(&mut self, len: usize) -> Result<&'a [u8]> {
        let end = self
            .at
            .checked_add(len)
            .ok_or_else(|| Error::new("cursor overflow"))?;
        if end > self.data.len() {
            return Err(Error::new("truncated frame"));
        }
        let slice = &self.data[self.at..end];
        self.at = end;
        Ok(slice)
    }

    pub fn u8(&mut self) -> Result<u8> {
        Ok(self.take(1)?[0])
    }

    pub fn u32(&mut self) -> Result<u32> {
        let bytes = self.take(4)?;
        Ok(u32::from_le_bytes(bytes.try_into().expect("4 bytes")))
    }

    pub fn hash(&mut self) -> Result<Hash> {
        let bytes = self.take(Hash::LEN)?;
        let mut raw = [0u8; Hash::LEN];
        raw.copy_from_slice(bytes);
        Ok(Hash::from_bytes(raw))
    }

    pub fn bytes(&mut self, len: usize) -> Result<Vec<u8>> {
        Ok(self.take(len)?.to_vec())
    }

    pub fn finish(self) -> Result<()> {
        if self.at != self.data.len() {
            return Err(Error::new("trailing frame bytes"));
        }
        Ok(())
    }
}

pub fn push_hash(out: &mut Vec<u8>, hash: &Hash) {
    out.extend_from_slice(hash.as_bytes());
}

pub fn push_u32(out: &mut Vec<u8>, value: u32) {
    out.extend_from_slice(&value.to_le_bytes());
}

pub fn encode_offer(id: &Hash, canonical: &[u8], hashes: &[Hash]) -> Result<Vec<u8>> {
    let mut body = Vec::new();
    push_hash(&mut body, id);
    push_len(&mut body, canonical.len())?;
    body.extend_from_slice(canonical);
    push_count(&mut body, hashes.len())?;
    for hash in hashes {
        push_hash(&mut body, hash);
    }
    Ok(body)
}

pub fn decode_offer(bytes: &[u8]) -> Result<(Hash, Vec<u8>, Vec<Hash>)> {
    let mut cursor = Cursor::new(bytes);
    let id = cursor.hash()?;
    let canon_len = cursor.u32()? as usize;
    let canonical = cursor.bytes(canon_len)?;
    let count = limited(cursor.u32()?)?;
    let mut hashes = Vec::with_capacity(count);
    for _ in 0..count {
        hashes.push(cursor.hash()?);
    }
    cursor.finish()?;
    Ok((id, canonical, hashes))
}

pub fn encode_hashes(hashes: &[Hash]) -> Result<Vec<u8>> {
    let mut body = Vec::new();
    push_count(&mut body, hashes.len())?;
    for hash in hashes {
        push_hash(&mut body, hash);
    }
    Ok(body)
}

pub fn encode_name(name: &str, offer: &[u8]) -> Result<Vec<u8>> {
    let mut body = Vec::new();
    push_len(&mut body, name.len())?;
    body.extend_from_slice(name.as_bytes());
    body.extend_from_slice(offer);
    Ok(body)
}

pub fn decode_name(bytes: &[u8]) -> Result<(String, Vec<u8>)> {
    let mut cursor = Cursor::new(bytes);
    let len = cursor.u32()? as usize;
    if len > 64 {
        return Err(Error::new("campaign name is too long").ctx("len", len));
    }
    let name = cursor.bytes(len)?;
    let name = String::from_utf8(name).map_err(|_| Error::new("campaign name is not utf-8"))?;
    let rest = cursor.data[cursor.at..].to_vec();
    Ok((name, rest))
}

pub fn decode_hashes(bytes: &[u8]) -> Result<Vec<Hash>> {
    let mut cursor = Cursor::new(bytes);
    let count = limited(cursor.u32()?)?;
    let mut hashes = Vec::with_capacity(count);
    for _ in 0..count {
        hashes.push(cursor.hash()?);
    }
    cursor.finish()?;
    Ok(hashes)
}

pub fn encode_bodies(objects: &[Body]) -> Result<Vec<u8>> {
    let mut body = Vec::new();
    push_count(&mut body, objects.len())?;
    for object in objects {
        push_hash(&mut body, &object.hash);
        body.push(object.kind);
        push_len(&mut body, object.bytes.len())?;
        body.extend_from_slice(&object.bytes);
    }
    Ok(body)
}

pub fn decode_bodies(bytes: &[u8]) -> Result<Vec<Body>> {
    let mut cursor = Cursor::new(bytes);
    let count = limited(cursor.u32()?)?;
    let mut objects = Vec::with_capacity(count);
    for _ in 0..count {
        let hash = cursor.hash()?;
        let kind = cursor.u8()?;
        let len = cursor.u32()? as usize;
        if len > MAX_BYTES as usize {
            return Err(Error::new("object exceeds 512MB").ctx("len", len));
        }
        objects.push(Body {
            hash,
            kind,
            bytes: cursor.bytes(len)?,
        });
    }
    cursor.finish()?;
    Ok(objects)
}

pub fn encode_done(id: &Hash, graph: &str) -> Result<Vec<u8>> {
    let flag = match graph {
        "inserted" => 1,
        "unchanged" => 0,
        other => return Err(Error::new("unknown graph result").ctx("graph", other)),
    };
    let mut body = Vec::new();
    push_hash(&mut body, id);
    body.push(flag);
    Ok(body)
}

pub fn decode_done(bytes: &[u8]) -> Result<(Hash, &'static str)> {
    let mut cursor = Cursor::new(bytes);
    let id = cursor.hash()?;
    let graph = match cursor.u8()? {
        1 => "inserted",
        0 => "unchanged",
        other => return Err(Error::new("unknown graph result").ctx("graph", other)),
    };
    cursor.finish()?;
    Ok((id, graph))
}

fn push_len(out: &mut Vec<u8>, len: usize) -> Result<()> {
    let count = u32::try_from(len).map_err(|_| Error::new("frame exceeds 4GB"))?;
    if count > MAX_BYTES {
        return Err(Error::new("frame exceeds 512MB").ctx("len", count));
    }
    push_u32(out, count);
    Ok(())
}

fn push_count(out: &mut Vec<u8>, len: usize) -> Result<()> {
    let count = u32::try_from(len).map_err(|_| Error::new("too many objects"))?;
    if count > MAX_OBJECTS {
        return Err(Error::new("too many objects").ctx("objects", count));
    }
    push_u32(out, count);
    Ok(())
}

fn limited(count: u32) -> Result<usize> {
    if count > MAX_OBJECTS {
        return Err(Error::new("too many objects").ctx("objects", count));
    }
    Ok(count as usize)
}

fn read_exact(io: &mut impl Read, buf: &mut [u8]) -> Result<()> {
    io.read_exact(buf)
        .map_err(|err| Error::new(err.to_string()).ctx("op", "read frame"))
}
