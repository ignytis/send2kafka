mod kafka_producer;

use std::env;

use actix_web::{post, App, HttpRequest, HttpServer, Result, web};
use log::info;

use kafka_producer::KafkaProducer;

struct AppState {
    producer: KafkaProducer,
}

impl AppState {
    fn new(producer: KafkaProducer) -> Self {
        AppState{producer}
    }
}

#[post("/{topic}")]
async fn index(req: HttpRequest, app_state: web::Data<AppState>, payload: web::Bytes) -> Result<String> {
    let topic: String = req.match_info().get("topic").unwrap().parse().unwrap();
    app_state.producer.produce(topic.as_str(), payload.to_vec()).await;
    Ok(String::from(""))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let http_host =  env::var("SEND2KAFKA_HTTP_HOST").unwrap_or(String::from("127.0.0.1"));
    let http_port = env::var("SEND2KAFKA_HTTP_PORT").unwrap_or(String::from("8080")).parse::<u16>().expect("Incorrect HTTP port provided");

    info!("Listening on {}:{}", http_host, http_port);
    HttpServer::new(|| {
        let kafka_bootstrap_servers = env::var("SEND2KAFKA_KAFKA_BROKERS").unwrap_or(String::from("localhost:9092"));
        let producer = kafka_producer::KafkaProducer::new(kafka_bootstrap_servers);

        App::new()
            .app_data(web::Data::new(AppState::new(producer)))
            .service(index)
    })
    .bind((http_host, http_port))?
    .run()
    .await
}