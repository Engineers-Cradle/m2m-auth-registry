use crate::config::env::get_env;
use crate::libs::db::{self, Node, NodeRegistration};
use crate::libs::http::AppState;
use crate::libs::jwt::{self, generate_jwt};
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
) -> actix_web::web::Json<jwt::VerifyNDecodedResult> {
    let register_token: Option<&str> = get_register_token(&req);

    let register_token: String = match register_token {
        Some(token) => token.to_string(),
        None => "".to_string(),
    };

    if register_token == "" {
        return actix_web::web::Json(jwt::VerifyNDecodedResult { 
            success: false,
            app_node: None,
            node_id: None,
        });
    }

    if register_token != get_env().registration_token {
        return actix_web::web::Json(jwt::VerifyNDecodedResult {
            success: false,
            app_node: None,
            node_id: None,
        });
    }

    let token = body.token.clone();

    if token == "" {
        return actix_web::web::Json(jwt::VerifyNDecodedResult {
            success:false,
            node_id: None,
            app_node: None, 
        });
    }

    let public_key = jwt::generate_public_key(data.private_key.clone());

    let jwt_result = jwt::verify_jwt(&token, public_key);

    actix_web::web::Json(jwt::VerifyNDecodedResult {
        success: jwt_result.success,
        app_node: jwt_result.app_node,
        node_id: jwt_result.node_id,
    })
}

pub fn init_data_routes(config: &mut web::ServiceConfig) {
    config.service(register_node);
    config.service(list_nodes);
    config.service(verify_node);
}
