use std::collections::BTreeMap;

use acvc_domain::{Hash, Result, validate_score};

use crate::{App, ScoreReport};

impl App {
    pub fn score(&self, variant: &Hash, metrics: &BTreeMap<String, f64>) -> Result<ScoreReport> {
        let stored = self.ports.variants.get_variant(variant)?;
        let schema = self.ports.campaigns.schema_of(&stored.campaign)?;
        validate_score(&schema, metrics)?;
        let trial = acvc_domain::Trial {
            variant: *variant,
            nonce: self.ports.clock.nonce(),
            metrics: metrics.clone(),
        };
        let id = self
            .ports
            .trials
            .insert_trial(&trial, self.ports.clock.now_millis())?;
        Ok(ScoreReport {
            trial: id,
            variant: *variant,
        })
    }

    pub fn reject(&self, variant: &Hash, reason: &str) -> Result<()> {
        self.ports
            .trials
            .reject(variant, reason, self.ports.clock.now_millis())
    }
}
