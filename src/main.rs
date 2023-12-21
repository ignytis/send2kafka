mod app_state;
mod http_api;
mod kafka_producer;

use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let http_host =  env::var("SEND2KAFKA_HTTP_HOST").unwrap_or(String::from("127.0.0.1"));
    let http_port = env::var("SEND2KAFKA_HTTP_PORT").unwrap_or(String::from("8080")).parse::<u16>().expect("Incorrect HTTP port provided");

    let kafka_bootstrap_servers = env::var("SEND2KAFKA_KAFKA_BROKERS").unwrap_or(String::from("localhost:9092"));

    http_api::start(http_host, http_port, kafka_bootstrap_servers).await
}