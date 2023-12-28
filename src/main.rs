use crate::api::admin::AdminApiHandler;
use crate::dal::*;
use actix_web::{App, HttpServer, web};
use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use dotenvy::dotenv;
use structured_logger::Builder;
use structured_logger::json::new_writer;
use std::env;
use std::io::stdout;

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

    Builder::with_level("debug")
        .with_target_writer("*", new_writer(stdout()))
        .init();

    let db_uri = env::var("DATABASE_URL").expect("Database url not defined");
    let manager = ConnectionManager::<PgConnection>::new(db_uri);
    let pool = Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool");

    let stub_dao = StubDao::new(pool.clone());
    let state_dao = StateDao::new(pool.clone());

    let admin_api_handler = AdminApiHandler::new(stub_dao, state_dao);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(admin_api_handler.clone()))
            .service(api::exec_get)
            .service(api::exec_post)
            .service(api::fetch_states)
            .service(api::create_stub)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}