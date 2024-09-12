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


pub async fn get_all_nodes(connection: &mut redis::aio::MultiplexedConnection) -> NodeList {
    // USE Keys to get all nodes
    let keys: Vec<String> = connection.keys("node_*").await.unwrap();

    let mut nodes: Vec<Node> = vec![];

    for key in keys {
        let id: String = connection.hget(&key, "id").await.unwrap();
        let app_node: String = connection.hget(&key, "app_node").await.unwrap();
        let last_token_issue_at: String =
            connection.hget(&key, "last_token_issue_at").await.unwrap();
        let last_ping_on: String = connection.hget(&key, "last_ping_on").await.unwrap();

        nodes.push(Node {
            id: id,
            app_node: app_node,
            last_token_issue_at: last_token_issue_at,
            last_ping_on: last_ping_on,
        });
    }

    NodeList { nodes: Some(nodes) }
}

pub async fn delete_node_by_name(
    connection: &mut redis::aio::MultiplexedConnection,
    app_node: &str,
) {
    let hash_key: String = "node_".to_owned() + &app_node;
    let _: () = connection.del(&hash_key).await.unwrap();
}
