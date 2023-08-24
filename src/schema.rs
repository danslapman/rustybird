// @generated automatically by Diesel CLI.

diesel::table! {
    stub (id) {
        id -> Int4,
        created -> Timestamptz,
        times -> Nullable<Int8>,
        #[max_length = 40]
        service_suffix -> Varchar,
        #[max_length = 40]
        name -> Varchar,
        #[max_length = 256]
        path -> Nullable<Varchar>,
        #[max_length = 256]
        path_pattern -> Nullable<Varchar>,
        seed -> Nullable<Jsonb>,
        state -> Nullable<Jsonb>,
        request -> Jsonb,
        persist -> Nullable<Jsonb>,
        response -> Jsonb,
    }
}
