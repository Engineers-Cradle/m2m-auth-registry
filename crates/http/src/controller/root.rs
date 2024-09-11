use actix_web::{get, web};
use serde::Serialize;

#[derive(Serialize)]
pub struct Status {
    pub status: String,
}

#[get("/health")]
async fn health() -> web::Json<Status> {
    web::Json(Status {
        status: "OK!".to_string(),
    })
}

pub fn init_root_routes(config: &mut web::ServiceConfig) {
    config.service(health);
}
