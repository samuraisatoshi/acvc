use crate::Hash;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Premise {
    pub claim: String,
    pub assumptions: Vec<String>,
    pub revises: Option<Hash>,
}
