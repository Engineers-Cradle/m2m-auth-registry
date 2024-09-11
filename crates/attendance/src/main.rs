mod config;
mod libs;

#[tokio::main]
async fn main() {
    let _ = libs::redis::start_pub_sub_attendence_marker().await;

    println!("PubSub Attendence started");
}
