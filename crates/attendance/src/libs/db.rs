use redis::AsyncCommands;
use serde::Deserialize;

#[derive(Deserialize, serde::Serialize)]
pub struct Node {
    pub id: String,
    pub app_node: String,
    pub last_token_issue_at: String,
    pub last_ping_on: String,
}

#[derive(Deserialize, serde::Serialize)]
pub struct NodeList {
    pub nodes: Option<Vec<Node>>,
}

#[derive(Deserialize, serde::Serialize)]
pub struct NodeRegistration {
    pub app_node: String,
}

// Get Node by Name
pub async fn get_node_by_name(
    connection: &mut redis::aio::MultiplexedConnection,
    app_node: &str,
) -> Node {
    let hash_key: String = "node_".to_owned() + &app_node;
    let id: String = connection.hget(&hash_key, "id").await.unwrap();
    let app_node: String = connection.hget(&hash_key, "app_node").await.unwrap();
    let last_token_issue_at: String = connection
        .hget(&hash_key, "last_token_issue_at")
        .await
        .unwrap();
    let last_ping_on: String = connection.hget(&hash_key, "last_ping_on").await.unwrap();

    Node {
        id: id,
        app_node: app_node,
        last_token_issue_at: last_token_issue_at,
        last_ping_on: last_ping_on,
    }
}

// Update last_token_issue_at Node by Name
pub async fn update_last_token_issue_at_node_by_name(
    connection: &mut redis::aio::MultiplexedConnection,
    app_node: &str,
    last_token_issue_at: &str,
) -> Node {
    let hash_key: String = "node_".to_owned() + &app_node;
    let _: () = connection
        .hset(&hash_key, "last_token_issue_at", last_token_issue_at)
        .await
        .unwrap();

    let id: String = connection.hget(&hash_key, "id").await.unwrap();
    let app_node: String = connection.hget(&hash_key, "app_node").await.unwrap();
    let last_token_issue_at: String = connection
        .hget(&hash_key, "last_token_issue_at")
        .await
        .unwrap();
    let last_ping_on: String = connection.hget(&hash_key, "last_ping_on").await.unwrap();

    Node {
        id: id,
        app_node: app_node,
        last_token_issue_at: last_token_issue_at,
        last_ping_on: last_ping_on,
    }
}

// Update last_ping_on Node by Name
pub async fn update_last_ping_on_node_by_name(
    connection: &mut redis::aio::MultiplexedConnection,
    app_node: &str,
    last_ping_on: &str,
) -> Node {
    let hash_key: String = "node_".to_owned() + &app_node;
    let _: () = connection
        .hset(&hash_key, "last_ping_on", last_ping_on)
        .await
        .unwrap();

    let id: String = connection.hget(&hash_key, "id").await.unwrap();
    let app_node: String = connection.hget(&hash_key, "app_node").await.unwrap();
    let last_token_issue_at: String = connection
        .hget(&hash_key, "last_token_issue_at")
        .await
        .unwrap();
    let last_ping_on: String = connection.hget(&hash_key, "last_ping_on").await.unwrap();

    Node {
        id: id,
        app_node: app_node,
        last_token_issue_at: last_token_issue_at,
        last_ping_on: last_ping_on,
    }
}