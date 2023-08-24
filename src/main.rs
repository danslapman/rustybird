use crate::api::resolver::StubResolver;
use actix_web::{App, HttpServer};

pub mod api;
pub mod model;
pub mod predicate_dsl;
pub mod utils;

//https://github.com/intellij-rust/intellij-rust/issues/5975
//#[allow(unused_must_use)]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(StubResolver {})
            .service(api::exec_get)
            .service(api::exec_post)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}