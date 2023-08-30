// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "http_method"))]
    pub struct HttpMethod;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "scope"))]
    pub struct Scope;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Scope;
    use super::sql_types::HttpMethod;

    stub (id) {
        id -> Int4,
        created -> Timestamptz,
        scope -> Scope,
        times -> Nullable<Int8>,
        #[max_length = 40]
        service_suffix -> Varchar,
        #[max_length = 40]
        name -> Varchar,
        method -> HttpMethod,
        #[max_length = 256]
        path -> Nullable<Varchar>,
        #[max_length = 256]
        path_pattern -> Nullable<Varchar>,
        seed -> Nullable<Jsonb>,
        state -> Nullable<Jsonb>,
        request -> Jsonb,
        persist -> Nullable<Jsonb>,
        response -> Jsonb,
        callback -> Nullable<Jsonb>,
    }
}
