mod app_state;
mod configuration;
mod http_api;
mod kafka_producer;

use std::env;

use config::Config as LibConfig;
use crate::configuration::Config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info")
    }
    env_logger::init();

    let cfg_file = env::var("SEND2KAFKA__CONFIG").unwrap_or(String::from("config.yaml"));
    let cfg = LibConfig::builder()
        .set_default("http.host", "0.0.0.0").unwrap()
        .set_default("http.port", "8080").unwrap()
        .set_default("http.num_workers", "0").unwrap()
        .set_default("kafka.bootstrap_servers", "localhost:9092").unwrap()
        .add_source(config::Environment::with_prefix("SEND2KAFKA").separator("__"))
        .add_source(config::File::with_name(cfg_file.as_str()).required(false))
        .build()
        .unwrap();

    let cfg: Config = match cfg.try_deserialize() {
        Ok(c) => c,
        Err(e) => panic!("Failed to build the configuration: {}", e),
    };

    http_api::start(cfg).await
}