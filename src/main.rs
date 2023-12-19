use std::env;
use std::time::Duration;

use actix_web::{post, App, HttpRequest, HttpServer, Result};
use log::info;

use rdkafka::config::ClientConfig;
use rdkafka::message::{Header, OwnedHeaders};
use rdkafka::producer::{FutureProducer, FutureRecord};


// An example from: https://github.com/fede1024/rust-rdkafka/blob/master/examples/simple_producer.rs

async fn produce(brokers: &str, topic_name: &str) {
    // TODO: do not initialize on after HTTP req. Move out if this function
    let producer: &FutureProducer = &ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

    // This loop is non blocking: all messages will be sent one after the other, without waiting
    // for the results.
    let futures = (0..5)
        .map(|i| async move {
            // The send operation on the topic returns a future, which will be
            // completed once the result or failure from Kafka is received.
            let delivery_status = producer
                .send(
                    FutureRecord::to(topic_name)
                    // TODO: replace with actual payload
                        .payload(&format!("Message {}", i))
                        .key(&format!("Key {}", i))
                        .headers(OwnedHeaders::new().insert(Header {
                            key: "header_key",
                            value: Some("header_value"),
                        })),
                    Duration::from_secs(0),
                )
                .await;

            // This will be executed when the result is received.
            delivery_status
        })
        .collect::<Vec<_>>();

    // This loop will wait until all delivery statuses have been received.
    for future in futures {
        info!("Future completed. Result: {:?}", future.await);
    }
}


#[post("/{topic}")]
async fn index(req: HttpRequest) -> Result<String> {
    
    let topic: String = req.match_info().get("topic").unwrap().parse().unwrap();
    // let topic = ""

    produce("localhost:29092", topic.as_str()).await;

    Ok(String::from(""))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let host =  env::var("SEND2KAFKA_HTTP_HOST").unwrap_or(String::from("127.0.0.1"));
    let port = env::var("SEND2KAFKA_HTTP_PORT").unwrap_or(String::from("8080")).parse::<u16>().expect("Incorrect HTTP port provided");
    info!("Listening on {}:{}", host, port);

    HttpServer::new(|| {
        App::new().service(index)
    })
    .bind((host, port))?
    .run()
    .await
}