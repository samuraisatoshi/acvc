use std::cmp::Ordering;
use std::collections::BTreeMap;

use crate::{Direction, Error, Hash, MetricSpec, Result, is_complete};

#[derive(Clone, Debug, PartialEq)]
pub struct ScoredVariant {
    pub variant: Hash,
    pub metrics: BTreeMap<String, f64>,
    pub rejected: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScoreRelation {
    LeftDominates,
    RightDominates,
    Tie,
    Incomparable,
    Incomplete,
}

pub fn compare_scores(
    schema: &[MetricSpec],
    left: &BTreeMap<String, f64>,
    right: &BTreeMap<String, f64>,
) -> Result<ScoreRelation> {
    if schema.is_empty() {
        return Err(Error::new("campaign has no metrics"));
    }
    if !is_complete(schema, left) || !is_complete(schema, right) {
        return Ok(ScoreRelation::Incomplete);
    }
    let left_wins = dominates(schema, left, right);
    let right_wins = dominates(schema, right, left);
    Ok(match (left_wins, right_wins) {
        (true, false) => ScoreRelation::LeftDominates,
        (false, true) => ScoreRelation::RightDominates,
        (false, false) if equal(schema, left, right) => ScoreRelation::Tie,
        _ => ScoreRelation::Incomparable,
    })
}

pub fn frontier<'a>(
    schema: &[MetricSpec],
    items: &'a [ScoredVariant],
) -> Result<Vec<&'a ScoredVariant>> {
    if schema.is_empty() {
        return Err(Error::new("campaign has no metrics"));
    }
    let eligible: Vec<&ScoredVariant> = items
        .iter()
        .filter(|item| !item.rejected && is_complete(schema, &item.metrics))
        .collect();
    let mut chosen: Vec<&ScoredVariant> = eligible
        .iter()
        .copied()
        .filter(|item| {
            !eligible.iter().any(|other| {
                other.variant != item.variant && dominates(schema, &other.metrics, &item.metrics)
            })
        })
        .collect();
    chosen.sort_by_key(|item| item.variant);
    Ok(chosen)
}

fn dominates(
    schema: &[MetricSpec],
    left: &BTreeMap<String, f64>,
    right: &BTreeMap<String, f64>,
) -> bool {
    let mut strict = false;
    for spec in schema {
        let Some(left_value) = left.get(&spec.name) else {
            return false;
        };
        let Some(right_value) = right.get(&spec.name) else {
            return false;
        };
        match better(spec.direction, *left_value, *right_value) {
            Ordering::Less => return false,
            Ordering::Greater => strict = true,
            Ordering::Equal => {}
        }
    }
    strict
}

fn equal(
    schema: &[MetricSpec],
    left: &BTreeMap<String, f64>,
    right: &BTreeMap<String, f64>,
) -> bool {
    schema
        .iter()
        .all(|spec| left.get(&spec.name) == right.get(&spec.name))
}

fn better(direction: Direction, left: f64, right: f64) -> Ordering {
    let order = left.partial_cmp(&right).unwrap_or(Ordering::Equal);
    match direction {
        Direction::Higher => order,
        Direction::Lower => order.reverse(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn schema() -> Vec<MetricSpec> {
        vec![
            MetricSpec {
                name: "tests_passed".into(),
                direction: Direction::Higher,
            },
            MetricSpec {
                name: "latency_ms".into(),
                direction: Direction::Lower,
            },
        ]
    }

    fn scored(byte: u8, tests: f64, latency: f64, rejected: bool) -> ScoredVariant {
        ScoredVariant {
            variant: Hash::blob(&[byte]),
            metrics: BTreeMap::from([
                ("tests_passed".into(), tests),
                ("latency_ms".into(), latency),
            ]),
            rejected,
        }
    }

    #[test]
    fn pareto_keeps_ties_and_drops_partial_and_rejected() {
        let slow = scored(1, 10.0, 50.0, false);
        let fast = scored(2, 10.0, 40.0, false);
        let other = scored(3, 12.0, 45.0, false);
        let tie = scored(4, 10.0, 40.0, false);
        let mut partial = scored(5, 99.0, 1.0, false);
        partial.metrics.remove("latency_ms");
        let rejected = scored(6, 100.0, 1.0, true);
        let items = vec![
            slow,
            fast.clone(),
            other.clone(),
            tie.clone(),
            partial,
            rejected,
        ];
        let front = frontier(&schema(), &items).unwrap();
        let ids: Vec<_> = front.iter().map(|item| item.variant).collect();
        assert!(ids.contains(&fast.variant));
        assert!(ids.contains(&tie.variant));
        assert!(ids.contains(&other.variant));
        assert!(!ids.contains(&items[0].variant));
        assert_eq!(ids.len(), 3);
        assert_eq!(
            compare_scores(&schema(), &fast.metrics, &items[0].metrics).unwrap(),
            ScoreRelation::LeftDominates
        );
    }
}
