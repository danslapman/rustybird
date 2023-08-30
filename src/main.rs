use crate::api::resolver::StubResolver;
use crate::dal::*;
use actix_web::{App, HttpServer, web};
use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use dotenvy::dotenv;
use std::env;

#[macro_use]
extern crate diesel_autoincrement_new_struct;

pub mod api;
pub mod dal;
pub mod error;
pub mod model;
pub mod predicate_dsl;
pub mod schema;
pub mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().expect(".env file not found");

    let db_uri = env::var("DATABASE_URL").expect("Database url not defined");
    let manager = ConnectionManager::<PgConnection>::new(db_uri);
    let pool = Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(StubDao::new(pool.clone())))
            .app_data(StubResolver {})
            .service(api::exec_get)
            .service(api::exec_post)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}