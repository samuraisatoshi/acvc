use std::collections::BTreeSet;

use crate::{Error, Result};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Direction {
    Lower = 0,
    Higher = 1,
}

impl Direction {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Lower => "lower",
            Self::Higher => "higher",
        }
    }

    pub fn parse(text: &str) -> Result<Self> {
        match text {
            "lower" => Ok(Self::Lower),
            "higher" => Ok(Self::Higher),
            _ => Err(Error::new("unknown metric direction").ctx("direction", text)),
        }
    }

    pub fn from_u8(value: u8) -> Result<Self> {
        match value {
            0 => Ok(Self::Lower),
            1 => Ok(Self::Higher),
            _ => Err(Error::new("unknown metric direction").ctx("direction", value)),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MetricSpec {
    pub name: String,
    pub direction: Direction,
}

pub fn validate_schema(schema: &[MetricSpec]) -> Result<()> {
    if schema.is_empty() {
        return Err(Error::new("campaign needs at least one metric"));
    }
    let mut names = BTreeSet::new();
    for spec in schema {
        if spec.name.is_empty()
            || spec.name.contains(':')
            || spec.name.contains('=')
            || spec.name.contains(' ')
        {
            return Err(Error::new("invalid metric name").ctx("metric", spec.name.clone()));
        }
        if !names.insert(spec.name.as_str()) {
            return Err(Error::new("duplicate metric").ctx("metric", spec.name.clone()));
        }
    }
    Ok(())
}

pub fn validate_campaign_name(name: &str) -> Result<()> {
    let ok = !name.is_empty()
        && name.len() <= 64
        && name
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' || ch == '.');
    if ok {
        Ok(())
    } else {
        Err(Error::new("invalid campaign name").ctx("campaign", name))
    }
}
