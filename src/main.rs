use std::env;
use std::time::Duration;

use actix_web::{post, App, HttpRequest, HttpServer, Result, web};
use log::info;

use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};


// An example from: https://github.com/fede1024/rust-rdkafka/blob/master/examples/simple_producer.rs

struct AppState {
    producer: FutureProducer,
}

impl AppState {
    fn new(producer: FutureProducer) -> Self {
        AppState{producer}
    }
}

async fn produce(producer: &FutureProducer, topic_name: &str, payload: Vec<u8>) {
    let res = producer
        .send(FutureRecord::to(topic_name)
                .payload(&payload.to_vec())
                .key(&String::new()),
            Duration::from_secs(0)).await;
    info!("Result: {:?}", res);
}

#[post("/{topic}")]
async fn index(req: HttpRequest, data: web::Data<AppState>, payload: web::Bytes) -> Result<String> {
    let topic: String = req.match_info().get("topic").unwrap().parse().unwrap();
    produce(&data.producer, topic.as_str(), payload.to_vec()).await;
    Ok(String::from(""))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let http_host =  env::var("SEND2KAFKA_HTTP_HOST").unwrap_or(String::from("127.0.0.1"));
    let http_port = env::var("SEND2KAFKA_HTTP_PORT").unwrap_or(String::from("8080")).parse::<u16>().expect("Incorrect HTTP port provided");

    info!("Listening on {}:{}", http_host, http_port);
    HttpServer::new(|| {
        let kafka_bootstrap_servers = env::var("SEND2KAFKA_KAFKA_BROKERS").unwrap_or(String::from("localhost:29092"));

        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", kafka_bootstrap_servers)
            .set("message.timeout.ms", "5000")
            .create()
            .expect("Can't create a Kafka producer");

        App::new()
            .app_data(web::Data::new(AppState::new(producer)))
            .service(index)
    })
    .bind((http_host, http_port))?
    .run()
    .await
}