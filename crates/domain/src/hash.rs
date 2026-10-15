use std::fmt;

use crate::{Error, Result};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hash([u8; Self::LEN]);

impl Hash {
    pub const LEN: usize = 32;

    pub fn from_bytes(bytes: [u8; Self::LEN]) -> Self {
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; Self::LEN] {
        &self.0
    }

    pub fn of_tagged(tag: u8, bytes: &[u8]) -> Self {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&[tag]);
        hasher.update(bytes);
        Self(*hasher.finalize().as_bytes())
    }

    pub fn blob(content: &[u8]) -> Self {
        Self::of_tagged(0, content)
    }

    pub fn tree(canonical: &[u8]) -> Self {
        Self::of_tagged(1, canonical)
    }

    pub fn identity(canonical: &[u8]) -> Self {
        Self::of_tagged(2, canonical)
    }

    pub fn trial(canonical: &[u8]) -> Self {
        Self::of_tagged(3, canonical)
    }

    pub fn to_hex(self) -> String {
        const HEX: &[u8; 16] = b"0123456789abcdef";
        let mut out = String::with_capacity(Self::LEN * 2);
        for byte in self.0 {
            out.push(HEX[(byte >> 4) as usize] as char);
            out.push(HEX[(byte & 0xf) as usize] as char);
        }
        out
    }

    pub fn parse(text: &str) -> Result<Self> {
        let text = text.trim();
        if text.len() != Self::LEN * 2 {
            return Err(Error::new("hash must be 64 hex chars").ctx("len", text.len()));
        }
        let mut bytes = [0u8; Self::LEN];
        for index in 0..Self::LEN {
            bytes[index] = u8::from_str_radix(&text[index * 2..index * 2 + 2], 16)
                .map_err(|_| Error::new("invalid hex").ctx("hash", text))?;
        }
        Ok(Self(bytes))
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_hex())
    }
}

impl fmt::Debug for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Hash({})", self.to_hex())
    }
}
