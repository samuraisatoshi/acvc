use crate::{Error, Hash, Result};

pub(crate) fn push_u8(out: &mut Vec<u8>, value: u8) {
    out.push(value);
}

pub(crate) fn push_u32(out: &mut Vec<u8>, value: u32) {
    out.extend_from_slice(&value.to_le_bytes());
}

pub(crate) fn push_u64(out: &mut Vec<u8>, value: u64) {
    out.extend_from_slice(&value.to_le_bytes());
}

pub(crate) fn push_i64(out: &mut Vec<u8>, value: i64) {
    out.extend_from_slice(&value.to_le_bytes());
}

pub(crate) fn push_f64(out: &mut Vec<u8>, value: f64) {
    out.extend_from_slice(&value.to_le_bytes());
}

pub(crate) fn push_str(out: &mut Vec<u8>, value: &str) {
    push_u32(out, value.len() as u32);
    out.extend_from_slice(value.as_bytes());
}

pub(crate) fn push_hash(out: &mut Vec<u8>, hash: &Hash) {
    out.extend_from_slice(hash.as_bytes());
}

pub(crate) struct Cursor<'a> {
    data: &'a [u8],
    at: usize,
}

impl<'a> Cursor<'a> {
    pub(crate) fn new(data: &'a [u8]) -> Self {
        Self { data, at: 0 }
    }

    fn take(&mut self, len: usize) -> Result<&'a [u8]> {
        let end = self
            .at
            .checked_add(len)
            .ok_or_else(|| Error::new("cursor overflow"))?;
        if end > self.data.len() {
            return Err(Error::new("truncated record"));
        }
        let slice = &self.data[self.at..end];
        self.at = end;
        Ok(slice)
    }

    pub(crate) fn u8(&mut self) -> Result<u8> {
        Ok(self.take(1)?[0])
    }

    pub(crate) fn u32(&mut self) -> Result<u32> {
        let bytes = self.take(4)?;
        Ok(u32::from_le_bytes(bytes.try_into().expect("4 bytes")))
    }

    pub(crate) fn u64(&mut self) -> Result<u64> {
        let bytes = self.take(8)?;
        Ok(u64::from_le_bytes(bytes.try_into().expect("8 bytes")))
    }

    pub(crate) fn i64(&mut self) -> Result<i64> {
        let bytes = self.take(8)?;
        Ok(i64::from_le_bytes(bytes.try_into().expect("8 bytes")))
    }

    pub(crate) fn hash(&mut self) -> Result<Hash> {
        let bytes = self.take(Hash::LEN)?;
        let mut raw = [0u8; Hash::LEN];
        raw.copy_from_slice(bytes);
        Ok(Hash::from_bytes(raw))
    }

    pub(crate) fn string(&mut self) -> Result<String> {
        let len = self.u32()? as usize;
        let bytes = self.take(len)?;
        String::from_utf8(bytes.to_vec()).map_err(|_| Error::new("string is not utf-8"))
    }

    pub(crate) fn finish(self) -> Result<()> {
        if self.at != self.data.len() {
            return Err(Error::new("trailing bytes"));
        }
        Ok(())
    }
}
