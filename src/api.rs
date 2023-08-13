use rocket::http::{Status, ContentType};
use std::path::PathBuf;

pub mod model;
pub mod guards;

#[get("/api/rustybird/exec/<path..>")]
pub fn exec_get(path: PathBuf) -> (Status, (ContentType, String)) {
    let path_str = path.into_os_string().into_string().unwrap();

    (Status::Ok, (ContentType::Text, path_str))
}