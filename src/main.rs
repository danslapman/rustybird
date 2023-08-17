#[macro_use] extern crate rocket;

pub mod api;
pub mod utils;

//https://github.com/intellij-rust/intellij-rust/issues/5975
#[rocket::main]
#[allow(unused_must_use)]
async fn main() {
    rocket::build().mount("/", routes![api::exec_get, api::exec_post]).launch().await;
}