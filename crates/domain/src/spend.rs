#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Spend {
    pub agent: String,
    pub tokens: u64,
    pub note: String,
    pub created_at: i64,
}
