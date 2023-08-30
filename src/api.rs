use crate::api::model::*;
use crate::api::admin::AdminApiHandler;
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Deserialize;

pub mod admin;
pub mod model;
pub mod resolver;

#[derive(Deserialize)]
pub struct PathInfo {
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

// ******************** Admin API ********************

#[post("/api/internal/rustybird/stub")]
pub async fn create_stub(req: web::Json<CreateStubRequest>, handler: web::Data<AdminApiHandler>) -> impl Responder {
    match handler.create_stub(req.into_inner()).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::UnprocessableEntity()
    }
}