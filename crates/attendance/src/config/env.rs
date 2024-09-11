use dotenv::dotenv;
use std::env;

pub struct Env {
    pub redis_url: String,
}

impl Env {
    pub fn new() -> Self {
        dotenv().ok();

        Self {
            redis_url: env::var("REDIS_URL").expect("REDIS_URL must be set"),
        }
    }
}

pub fn get_env() -> Env {
    Env::new()
}
