use serde::{Deserialize, Serialize};
pub struct Dataset {
    id: String,
}

#[derive(Serialize, Deserialize)]
pub enum FilterPredicate<T>
where
    T: PartialEq + PartialOrd,
{
    Eq(T),
    Lt(T),
    Lte(T),
    Gt(T),
    Gte(T),
    Ne(T),
}
