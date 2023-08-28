use diesel_derive_enum;

pub mod persistent;

#[derive(Debug, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::Scope"]
pub enum Scope {
    Persistent,
    Ephemeral,
    Countdown
}

#[derive(Debug, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::HttpMethod"]
pub enum HttpMethod {
    Get,
    Post,
    Head,
    Options,
    Patch,
    Put,
    Delete
}