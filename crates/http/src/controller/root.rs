use actix_web::{get, web};
use serde::Serialize;
use crate::libs::{http::AppState, jwt::generate_jwks};
use jwtk;


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

#[derive(Serialize)]
pub struct Jwks {
    keys: Vec<jwtk::jwk::Jwk>,
}

#[get("/jwks.json")]
async fn jwks(
    data: web::Data<AppState>,
) -> web::Json<Jwks> {
    let private_key = data.private_key.clone();
    let jwks: jwtk::jwk::Jwk = generate_jwks(private_key);

    web::Json(Jwks {
        keys: vec![jwks],
    })
}

pub fn init_root_routes(config: &mut web::ServiceConfig) {
    config.service(health);
    config.service(jwks);
}
