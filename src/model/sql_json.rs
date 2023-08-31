use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub enum Keyword {
    #[serde(rename = "==")]
    Eq,
    #[serde(rename = "!=")]
    NotEq,
    #[serde(rename = "<")]
    Less,
    #[serde(rename = "<=")]
    Lte,
    #[serde(rename = ">")]
    Greater,
    #[serde(rename = ">=")]
    Gte,
    #[serde(rename = "~=")]
    Like,
    #[serde(rename = "^")]
    StartsWith,
    #[serde(rename = "exists")]
    Exists
}