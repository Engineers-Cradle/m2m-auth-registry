use hyperflake_rs::snowflake::SnowflakeId;
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

pub async fn insert_new_node(
    connection: &mut redis::aio::MultiplexedConnection,
    r_node: &NodeRegistration,
) -> Node {
    let id = SnowflakeId::new().generate();
    let app_node = r_node.app_node.clone();
    let last_token_issue_at = chrono::Utc::now().timestamp().to_string();
    let last_ping_on = chrono::Utc::now().timestamp().to_string();

    let hash_key: String = "node_".to_owned() + &app_node;
    let _: () = connection
        .hset_multiple(
            hash_key,
            &[
                ("id", id.clone()),
                ("app_node", app_node.clone()),
                ("last_token_issue_at", last_token_issue_at.clone()),
                ("last_ping_on", last_ping_on.clone()),
            ],
        )
        .await
        .unwrap();

    Node {
        id: id,
        app_node: app_node,
        last_token_issue_at: last_token_issue_at,
        last_ping_on: last_ping_on,
    }
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