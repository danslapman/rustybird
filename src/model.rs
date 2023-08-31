use diesel_derive_enum;
use serde::{Deserialize, Serialize};

pub mod persistent;
pub mod sql_json;

#[derive(Debug, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::Scope"]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Scope {
    Persistent,
    Ephemeral,
    Countdown
}

#[derive(Debug, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::HttpMethod"]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    Get,
    Post,
    Head,
    Options,
    Patch,
    Put,
    Delete
}