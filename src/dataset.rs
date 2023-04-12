use serde::{Deserialize, Serialize};
pub struct Dataset {
    id: String,
}

#[derive(Serialize, Deserialize)]
pub enum FilterPredicate {
    Eq(String),
    Lt(String),
    Lte(String),
    Gt(String),
    Gte(String),
    Ne(String),
}
