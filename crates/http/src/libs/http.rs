use crate::controller::data;
use crate::controller::root;
use crate::libs::jwt;

use actix_web::{web, App, HttpServer};
use env_logger::Env;
use listenfd::ListenFd;
use tracing_actix_web::TracingLogger;

#[derive(Clone)]
pub struct AppState {
    pub redis_client: redis::Client,
    pub private_key: jwtk::ecdsa::EcdsaPrivateKey,
}

#[actix_web::main]
pub async fn start_web_server() -> std::io::Result<()> {
    let env_config = crate::config::env::get_env();

    let redis_client: redis::Client =
        crate::libs::redis::connection_to_redis(&env_config.redis_url).await;

    let private_key = jwt::generate_private_key();
    let app_state: AppState = AppState {
        redis_client: redis_client,
        private_key: private_key,
    };

    env_logger::init_from_env(Env::default().default_filter_or(env_config.log_level));

    let mut listenfd = ListenFd::from_env();

    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(TracingLogger::default())
            .configure(root::init_root_routes)
            .configure(data::init_data_routes)
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = "0.0.0.0";
            let port = env_config.web_server_port;

            println!("Web Server started at http://{}:{}", host, port);

            server.bind(format!("{}:{}", host, port))?
        }
    };

    server.workers(env_config.num_workers).run().await
}
