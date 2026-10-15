use crate::codec::{Cursor, push_str};
use crate::{Error, Hash, Result};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Marker {
    pub repo: String,
    pub campaign: String,
    pub variant: Hash,
}

impl Marker {
    pub fn encode(&self) -> Result<Vec<u8>> {
        if self.repo.contains('\n') || self.campaign.contains('\n') {
            return Err(Error::new("marker fields cannot contain newlines"));
        }
        let mut out = Vec::new();
        push_str(&mut out, "ACVCMARK1");
        push_str(&mut out, &self.repo);
        push_str(&mut out, &self.campaign);
        push_str(&mut out, &self.variant.to_hex());
        Ok(out)
    }

    pub fn decode(bytes: &[u8]) -> Result<Self> {
        let mut cursor = Cursor::new(bytes);
        if cursor.string()? != "ACVCMARK1" {
            return Err(Error::new("bad workspace marker"));
        }
        let marker = Self {
            repo: cursor.string()?,
            campaign: cursor.string()?,
            variant: Hash::parse(&cursor.string()?)?,
        };
        cursor.finish()?;
        Ok(marker)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn marker_roundtrip() {
        let marker = Marker {
            repo: "/repo".into(),
            campaign: "search".into(),
            variant: Hash::blob(b"v"),
        };
        assert_eq!(Marker::decode(&marker.encode().unwrap()).unwrap(), marker);
    }
}
