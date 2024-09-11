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

    pubsub.psubscribe("m2m:auth:ping").unwrap();

    loop {
        let msg = pubsub.get_message().unwrap();
        let payload: String = msg.get_payload().unwrap();

        let payload_parts: Vec<&str> = payload.split(":").collect();
        let node_id: &str = payload_parts[0];

        let key: String = format!("m2m:auth:{}", &node_id);

        let mut redis_multiplex_connection: redis::aio::MultiplexedConnection = redis_client
            .get_multiplexed_async_connection()
            .await
            .unwrap();

        let _: () = redis_multiplex_connection
            .publish(key, "pong")
            .await
            .unwrap();

        println!("Ponged to {}", node_id);

        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}
