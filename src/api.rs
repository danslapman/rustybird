use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Deserialize;

pub mod model;
pub mod resolver;

#[derive(Deserialize)]
struct PathInfo {
    path: String
}

#[get("/api/rustybird/exec/{path:.*}")]
pub async fn exec_get(path: web::Path<PathInfo>) -> impl Responder {
    HttpResponse::Ok().body(format!("{}", path.path))
}

#[post("/api/rustybird/exec/{path:.*}")]
pub async fn exec_post(path: web::Path<PathInfo>, body: String) -> impl Responder {
    HttpResponse::Ok().body(format!("{} {}", path.path, body))
}