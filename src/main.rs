#[macro_use] extern crate rocket;

pub mod utils;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

//https://github.com/intellij-rust/intellij-rust/issues/5975
#[rocket::main]
#[allow(unused_must_use)]
async fn main() {
    rocket::build().mount("/", routes![index]).launch().await;
}