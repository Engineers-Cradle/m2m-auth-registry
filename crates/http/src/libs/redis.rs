pub async fn connection_to_redis(redis_url: &str) -> redis::Client {
    let client: redis::Client = redis::Client::open(redis_url).unwrap();
    client
}