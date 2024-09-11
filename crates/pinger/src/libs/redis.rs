use redis::AsyncCommands;

pub async fn connection_to_redis(redis_url: &str) -> redis::Client {
    let client: redis::Client = redis::Client::open(redis_url).unwrap();
    client
}

pub async fn start_pub_sub_pinger() {
    let env_config = crate::config::env::get_env();

    let redis_client: redis::Client =
        crate::libs::redis::connection_to_redis(&env_config.redis_url).await;

    let mut connection = redis_client.clone().get_connection().unwrap();
    let mut pubsub = connection.as_pubsub();

    pubsub.psubscribe("m2m:auth").unwrap();

    println!("Pinger started");

    loop {
        let mut redis_multiplex_connection: redis::aio::MultiplexedConnection = redis_client
            .get_multiplexed_async_connection()
            .await
            .unwrap();

        let all_nodes = crate::libs::db::get_all_nodes(&mut redis_multiplex_connection).await;

        for node in all_nodes.nodes.unwrap() {
            let key: String = format!("m2m:auth:{}", &node.app_node);
            let _: () = redis_multiplex_connection
                .publish(key, "ping")
                .await
                .unwrap();
        }

        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}
