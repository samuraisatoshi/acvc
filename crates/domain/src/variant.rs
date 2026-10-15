use crate::codec::{push_hash, push_str, push_u8, push_u32};
use crate::{Error, Hash, Premise, Result};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EdgeKind {
    Seed = 0,
    Mutate = 1,
    Revert = 2,
    RevisePremise = 3,
    Synthesize = 4,
}

impl EdgeKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Seed => "seed",
            Self::Mutate => "mutate",
            Self::Revert => "revert",
            Self::RevisePremise => "revise_premise",
            Self::Synthesize => "synthesize",
        }
    }

    pub fn parse(text: &str) -> Result<Self> {
        match text {
            "seed" => Ok(Self::Seed),
            "mutate" => Ok(Self::Mutate),
            "revert" => Ok(Self::Revert),
            "revise_premise" => Ok(Self::RevisePremise),
            "synthesize" => Ok(Self::Synthesize),
            _ => Err(Error::new("unknown edge").ctx("edge", text)),
        }
    }

    pub fn from_u8(value: u8) -> Result<Self> {
        match value {
            0 => Ok(Self::Seed),
            1 => Ok(Self::Mutate),
            2 => Ok(Self::Revert),
            3 => Ok(Self::RevisePremise),
            4 => Ok(Self::Synthesize),
            _ => Err(Error::new("unknown edge").ctx("edge", value)),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variant {
    pub tree: Hash,
    pub parents: Vec<Hash>,
    pub edge: EdgeKind,
    pub premise: Premise,
    pub intent: String,
    pub agent: String,
}

impl Variant {
    pub fn validate(&self) -> Result<()> {
        if self.premise.claim.is_empty() {
            return Err(Error::new("premise claim is empty"));
        }
        match self.edge {
            EdgeKind::Seed if !self.parents.is_empty() => {
                Err(Error::new("seed cannot have parents").ctx("parents", self.parents.len()))
            }
            EdgeKind::Mutate | EdgeKind::Revert if self.parents.len() != 1 => {
                Err(Error::new("edge requires one parent")
                    .ctx("edge", self.edge.as_str())
                    .ctx("parents", self.parents.len()))
            }
            EdgeKind::RevisePremise
                if self.parents.len() != 1 || self.premise.revises.is_none() =>
            {
                Err(Error::new("revise_premise requires one parent and revises")
                    .ctx("parents", self.parents.len()))
            }
            EdgeKind::Synthesize if self.parents.len() < 2 => {
                Err(Error::new("synthesize requires two or more parents")
                    .ctx("parents", self.parents.len()))
            }
            _ => Ok(()),
        }
    }

    pub fn canonical(&self) -> Result<Vec<u8>> {
        self.validate()?;
        let mut out = Vec::new();
        push_hash(&mut out, &self.tree);
        push_u32(&mut out, self.parents.len() as u32);
        for parent in &self.parents {
            push_hash(&mut out, parent);
        }
        push_u8(&mut out, self.edge as u8);
        push_str(&mut out, &self.premise.claim);
        push_u32(&mut out, self.premise.assumptions.len() as u32);
        for assumption in &self.premise.assumptions {
            push_str(&mut out, assumption);
        }
        match self.premise.revises {
            Some(hash) => {
                push_u8(&mut out, 1);
                push_hash(&mut out, &hash);
            }
            None => push_u8(&mut out, 0),
        }
        push_str(&mut out, &self.intent);
        push_str(&mut out, &self.agent);
        Ok(out)
    }

    pub fn id(&self) -> Result<Hash> {
        Ok(Hash::identity(&self.canonical()?))
    }

    pub fn decode(bytes: &[u8]) -> Result<Self> {
        let mut cursor = crate::codec::Cursor::new(bytes);
        let tree = cursor.hash()?;
        let parent_count = cursor.u32()? as usize;
        if parent_count > 1024 {
            return Err(Error::new("too many parents").ctx("parents", parent_count));
        }
        let mut parents = Vec::with_capacity(parent_count);
        for _ in 0..parent_count {
            parents.push(cursor.hash()?);
        }
        let edge = EdgeKind::from_u8(cursor.u8()?)?;
        let claim = cursor.string()?;
        let assumption_count = cursor.u32()? as usize;
        if assumption_count > 1024 {
            return Err(Error::new("too many assumptions").ctx("assumptions", assumption_count));
        }
        let mut assumptions = Vec::with_capacity(assumption_count);
        for _ in 0..assumption_count {
            assumptions.push(cursor.string()?);
        }
        let revises = match cursor.u8()? {
            0 => None,
            1 => Some(cursor.hash()?),
            other => return Err(Error::new("bad revises flag").ctx("flag", other)),
        };
        let variant = Self {
            tree,
            parents,
            edge,
            premise: Premise {
                claim,
                assumptions,
                revises,
            },
            intent: cursor.string()?,
            agent: cursor.string()?,
        };
        cursor.finish()?;
        variant.validate()?;
        if variant.canonical()? != bytes {
            return Err(Error::new("variant encoding is not canonical"));
        }
        Ok(variant)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StoredVariant {
    pub campaign: String,
    pub variant: Variant,
    pub created_at: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample(edge: EdgeKind, parents: Vec<Hash>) -> Variant {
        Variant {
            tree: Hash::blob(b"tree"),
            parents,
            edge,
            premise: Premise {
                claim: "claim".into(),
                assumptions: vec!["a".into()],
                revises: None,
            },
            intent: "try".into(),
            agent: "agent".into(),
        }
    }

    #[test]
    fn seed_rejects_parents_and_id_is_stable() {
        let parent = Hash::blob(b"parent");
        assert!(sample(EdgeKind::Seed, vec![parent]).validate().is_err());
        let variant = sample(EdgeKind::Seed, vec![]);
        assert_eq!(variant.id().unwrap(), variant.id().unwrap());
        let encoded = variant.canonical().unwrap();
        assert_eq!(Variant::decode(&encoded).unwrap(), variant);
        let mut changed = variant.clone();
        changed.intent = "other".into();
        assert_ne!(variant.id().unwrap(), changed.id().unwrap());
    }

    #[test]
    fn synthesize_needs_two_parents() {
        let parent = Hash::blob(b"p");
        assert!(
            sample(EdgeKind::Synthesize, vec![parent])
                .validate()
                .is_err()
        );
        let variant = sample(
            EdgeKind::Synthesize,
            vec![Hash::blob(b"a"), Hash::blob(b"b")],
        );
        assert!(variant.id().is_ok());
    }
}
