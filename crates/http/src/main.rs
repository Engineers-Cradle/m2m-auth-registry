mod config;
mod controller;
mod libs;

#[tokio::main]
async fn main() {
    tokio::task::spawn_blocking(|| {
        let _ = libs::http::start_web_server();

        println!("Web Server started");
    })
    .await
    .unwrap();
}
