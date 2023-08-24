use rocket::http::{Status, ContentType};
use std::path::PathBuf;
use crate::api::guards::*;

pub mod model;
pub mod guards;
pub mod resolver;

#[get("/api/rustybird/exec/<path..>")]
pub fn exec_get(path: PathBuf, headers: RequestHeaders<'_>, query: QueryParameters<'_>) -> (Status, (ContentType, String)) {
    let path_str = path.into_os_string().into_string().unwrap();

    (Status::Ok, (ContentType::Text, path_str))
}

#[post("/api/rustybird/exec/<path..>", data = "<body>")]
pub fn exec_post(path: PathBuf, headers: RequestHeaders<'_>, query: QueryParameters<'_>, body: String) -> (Status, (ContentType, String)) {
    let path_str = path.into_os_string().into_string().unwrap();

    (Status::Ok, (ContentType::Text, format!("{} {}", path_str, body)))
}