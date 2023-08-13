use rocket::http::{Status, ContentType};
use std::path::PathBuf;
use crate::api::guards::*;

pub mod model;
pub mod guards;

#[get("/api/rustybird/exec/<path..>")]
pub fn exec_get(path: PathBuf, headers: RequestHeaders<'_>) -> (Status, (ContentType, String)) {
    let path_str = path.into_os_string().into_string().unwrap();

    (Status::Ok, (ContentType::Text, path_str))
}