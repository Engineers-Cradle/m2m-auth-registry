use crate::libs::db::{update_last_ping_on_node_by_name, update_last_token_issue_at_node_by_name};
use redis::AsyncCommands;

pub async fn connection_to_redis(redis_url: &str) -> redis::Client {
    let client: redis::Client = redis::Client::open(redis_url).unwrap();
    client
}

pub async fn start_pub_sub_attendence_marker() {
    let env_config = crate::config::env::get_env();

    let redis_client: redis::Client =
        crate::libs::redis::connection_to_redis(&env_config.redis_url).await;

    let mut connection = redis_client.clone().get_connection().unwrap();
    let mut pubsub = connection.as_pubsub();

    pubsub.psubscribe("m2m:auth:mark_attendance").unwrap();

    let private_key = crate::libs::jwt::read_private_key();

    loop {
        let msg = pubsub.get_message().unwrap();
        let payload: String = msg.get_payload().unwrap();

        let payload_parts: Vec<&str> = payload.split(":").collect();
        let node_name: &str = payload_parts[0];

        let mut redis_multiplex_connection: redis::aio::MultiplexedConnection = redis_client
            .get_multiplexed_async_connection()
            .await
            .unwrap();

        println!("Marked attendance for {}", node_name);

        let current_time = chrono::Utc::now().timestamp().to_string();
        let node_info: crate::libs::db::Node = crate::libs::db::get_node_by_name(&mut redis_multiplex_connection, node_name).await;

        println!("Last token issue at: {}", node_info.last_token_issue_at);

        let last_token_issue_at: i64 = node_info.last_token_issue_at.parse().unwrap();
        let current_time: i64 = current_time.parse().unwrap();

        if current_time - last_token_issue_at > 5400 {
            println!("Issuing new token for {}", node_name);

            let new_token = crate::libs::jwt::generate_jwt(
                node_info.id.to_owned(),
                node_name.to_owned(),
                &private_key
            );

            update_last_token_issue_at_node_by_name(
                &mut redis_multiplex_connection,
                node_name,
                &current_time.to_string()
            )
                .await;

            let _: () = redis_multiplex_connection
                .publish(format!("m2m:auth:grant_token:{}", node_name) , 
                    new_token
                )
                .await
                .unwrap();

            println!("New token issued for {}", node_name);        
        }

        update_last_ping_on_node_by_name(
            &mut redis_multiplex_connection,
            node_name,
            &current_time.to_string()
        )
            .await;
    }
}
