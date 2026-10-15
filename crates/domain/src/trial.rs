use std::collections::BTreeMap;

use crate::codec::{push_f64, push_hash, push_str, push_u32, push_u64};
use crate::{Error, Hash, MetricSpec, Result};

#[derive(Clone, Debug, PartialEq)]
pub struct Trial {
    pub variant: Hash,
    pub nonce: u64,
    pub metrics: BTreeMap<String, f64>,
}

impl Trial {
    pub fn canonical(&self) -> Vec<u8> {
        let mut out = Vec::new();
        push_hash(&mut out, &self.variant);
        push_u64(&mut out, self.nonce);
        push_u32(&mut out, self.metrics.len() as u32);
        for (name, value) in &self.metrics {
            push_str(&mut out, name);
            push_f64(&mut out, *value);
        }
        out
    }

    pub fn id(&self) -> Hash {
        Hash::trial(&self.canonical())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct StoredTrial {
    pub trial: Trial,
    pub created_at: i64,
}

pub fn validate_score(schema: &[MetricSpec], metrics: &BTreeMap<String, f64>) -> Result<()> {
    for (name, value) in metrics {
        if !value.is_finite() {
            return Err(Error::new("metric is not finite").ctx("metric", name));
        }
        if !schema.iter().any(|spec| spec.name == *name) {
            return Err(Error::new("unknown metric").ctx("metric", name));
        }
    }
    Ok(())
}

pub fn is_complete(schema: &[MetricSpec], metrics: &BTreeMap<String, f64>) -> bool {
    schema.iter().all(|spec| metrics.contains_key(&spec.name))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{EdgeKind, Premise, Variant};

    #[test]
    fn trial_does_not_change_variant_id() {
        let variant = Variant {
            tree: Hash::blob(b"t"),
            parents: vec![],
            edge: EdgeKind::Seed,
            premise: Premise {
                claim: "c".into(),
                assumptions: vec![],
                revises: None,
            },
            intent: String::new(),
            agent: "a".into(),
        };
        let id = variant.id().unwrap();
        let trial = Trial {
            variant: id,
            nonce: 9,
            metrics: BTreeMap::from([("tests_passed".into(), 1.0)]),
        };
        assert_ne!(trial.id(), id);
        assert_eq!(variant.id().unwrap(), id);
        assert!(
            !variant
                .canonical()
                .unwrap()
                .windows(12)
                .any(|w| w == b"tests_passed")
        );
    }
}
