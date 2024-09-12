use redis::AsyncCommands;

use crate::libs::db::delete_node_by_name;

pub async fn connection_to_redis(redis_url: &str) -> redis::Client {
    let client: redis::Client = redis::Client::open(redis_url).unwrap();
    client
}

pub async fn start_pub_sub_pinger() {
    let env_config = crate::config::env::get_env();

    let redis_client: redis::Client =
        crate::libs::redis::connection_to_redis(&env_config.redis_url).await;

    println!("Pinger started");

    loop {
        let mut redis_multiplex_connection: redis::aio::MultiplexedConnection = redis_client
            .get_multiplexed_async_connection()
            .await
            .unwrap();

        let all_nodes = crate::libs::db::get_all_nodes(&mut redis_multiplex_connection).await;

        for node in all_nodes.nodes.unwrap() {
            let key: String = format!("m2m:auth:ping:{}", &node.app_node);
            let last_ping_on: String = redis_multiplex_connection
                .hget(&format!("node_{}", &node.app_node), "last_ping_on")
                .await
                .unwrap();

            let last_ping_on: i64 = last_ping_on.parse().unwrap();
            let current_time: i64 = chrono::Utc::now().timestamp();

            if current_time - last_ping_on > 5 * 60 {
                println!("Node {} was not alive in last 5 mins, deleting ...", &node.app_node);
                delete_node_by_name(&mut redis_multiplex_connection, &node.app_node).await;
            }
            else {
                println!("Node {} was alive in last 5 mins", &node.app_node);
            }
            
            let _: () = redis_multiplex_connection
                .publish(key, "ping")
                .await
                .unwrap();
        }

        std::thread::sleep(std::time::Duration::from_secs(30));
    }
}
