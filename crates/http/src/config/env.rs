use dotenv::dotenv;
use std::env;

pub struct Env {
    pub redis_url: String,
    pub web_server_port: String,
    pub num_workers: usize,
    pub log_level: String,
    pub registration_token: String,
}

impl Env {
    pub fn new() -> Self {
        dotenv().ok();

        Self {
            redis_url: env::var("REDIS_URL").expect("REDIS_URL must be set"),
            web_server_port: env::var("WEB_SERVER_PORT").expect("WEB_SERVER_PORT must be set"),
            num_workers: env::var("NUM_WORKERS")
                .expect("NUM_WORKERS must be set")
                .parse::<usize>()
                .unwrap(),
            log_level: env::var("LOG_LEVEL").expect("LOG_LEVEL must be set"),
            registration_token: env::var("REGISTRATION_TOKEN")
                .expect("REGISTRATION_TOKEN must be set"),
        }
    }
}

pub fn get_env() -> Env {
    Env::new()
}
