use diesel_derive_enum;
use serde::{Deserialize, Serialize};

pub mod persistent;

#[derive(Debug, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::Scope"]
#[derive(Serialize, Deserialize)]
pub enum Scope {
    Persistent,
    Ephemeral,
    Countdown
}

#[derive(Debug, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::HttpMethod"]
#[derive(Serialize, Deserialize)]
pub enum HttpMethod {
    Get,
    Post,
    Head,
    Options,
    Patch,
    Put,
    Delete
}