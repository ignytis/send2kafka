mod app_state;
mod configuration;
mod http_api;
mod kafka_producer;

use config::Config as LibConfig;

use crate::configuration::Config;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let cfg = LibConfig::builder()
        .set_default("http.host", "127.0.0.1").unwrap()
        .set_default("http.port", "8080").unwrap()
        .set_default("kafka.bootstrap_servers", "localhost:9092").unwrap()
        .add_source(config::Environment::with_prefix("SEND2KAFKA").separator("__"))
        .add_source(config::File::with_name("config.yaml").required(false))
        .build()
        .unwrap();

    let cfg: Config = match cfg.try_deserialize() {
        Ok(c) => c,
        Err(e) => panic!("Failed to build the configuration: {}", e),
    };

    println!("{:?}", &cfg);

    http_api::start(cfg).await
}