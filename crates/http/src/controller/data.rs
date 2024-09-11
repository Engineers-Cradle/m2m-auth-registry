use crate::config::env::get_env;
use crate::libs::db::{self, Node, NodeRegistration};
use crate::libs::http::AppState;
use crate::libs::jwt::{generate_jwt, verify_jwt};
use actix_web::{get, post, web, HttpRequest};
use serde::Deserialize;

fn get_register_token<'a>(req: &'a HttpRequest) -> Option<&'a str> {
    req.headers().get("x-register-token")?.to_str().ok()
}

#[derive(Deserialize, serde::Serialize)]
struct RegistrationResult {
    success: bool,
    initial_token: Option<String>,
}

#[derive(Deserialize, serde::Serialize)]
struct NodeVerify {
    token: String,
}

#[derive(Deserialize, serde::Serialize)]
struct VerifyResult {
    success: bool,
}

#[derive(Deserialize, serde::Serialize)]
struct ListNodesResult {
    success: bool,
    nodes: Option<Vec<Node>>,
}

#[post("/node/register")]
async fn register_node(
    req: HttpRequest,
    body: web::Json<NodeRegistration>,
    data: web::Data<AppState>,
) -> actix_web::web::Json<RegistrationResult> {
    let register_token: Option<&str> = get_register_token(&req);
    let app_node = body.app_node.clone();

    let register_token: String = match register_token {
        Some(token) => token.to_string(),
        None => "".to_string(),
    };

    if register_token == "" {
        return actix_web::web::Json(RegistrationResult {
            success: false,
            initial_token: None,
        });
    }

    if register_token != get_env().registration_token {
        return actix_web::web::Json(RegistrationResult {
            success: false,
            initial_token: None,
        });
    }

    let mut redis_multiplex_connection: redis::aio::MultiplexedConnection = data
        .redis_client
        .get_multiplexed_async_connection()
        .await
        .unwrap();

    let insert_data: db::Node = db::insert_new_node(
        &mut redis_multiplex_connection,
        &db::NodeRegistration {
            app_node: app_node.clone(),
        },
    )
    .await;

    let initial_token: String = generate_jwt(
        insert_data.id.to_string(),
        insert_data.app_node.to_string(),
        &data.private_key,
    );

    actix_web::web::Json(RegistrationResult {
        success: true,
        initial_token: Some(initial_token),
    })
}

#[get("/node/list")]
async fn list_nodes(
    req: HttpRequest,
    data: web::Data<AppState>,
) -> actix_web::web::Json<ListNodesResult> {
    let register_token: Option<&str> = get_register_token(&req);

    let register_token: String = match register_token {
        Some(token) => token.to_string(),
        None => "".to_string(),
    };

    if register_token == "" {
        return actix_web::web::Json(ListNodesResult {
            success: false,
            nodes: None,
        });
    }

    let mut redis_multiplex_connection: redis::aio::MultiplexedConnection = data
        .redis_client
        .get_multiplexed_async_connection()
        .await
        .unwrap();

    let nodes: db::NodeList = db::get_all_nodes(&mut redis_multiplex_connection).await;

    actix_web::web::Json(ListNodesResult {
        success: true,
        nodes: nodes.nodes,
    })
}

#[post("/node/verify")]
async fn verify_node(
    req: HttpRequest,
    body: web::Json<NodeVerify>,
    data: web::Data<AppState>,
) -> actix_web::web::Json<VerifyResult> {
    let register_token: Option<&str> = get_register_token(&req);

    let register_token: String = match register_token {
        Some(token) => token.to_string(),
        None => "".to_string(),
    };

    if register_token == "" {
        return actix_web::web::Json(VerifyResult { success: false });
    }

    if register_token != get_env().registration_token {
        return actix_web::web::Json(VerifyResult { success: false });
    }

    let token = body.token.clone();

    if token == "" {
        return actix_web::web::Json(VerifyResult { success: false });
    }

    let jwt_result = verify_jwt(&token, &data.public_key);

    actix_web::web::Json(VerifyResult {
        success: jwt_result,
    })
}

pub fn init_data_routes(config: &mut web::ServiceConfig) {
    config.service(register_node);
    config.service(list_nodes);
    config.service(verify_node);
}
