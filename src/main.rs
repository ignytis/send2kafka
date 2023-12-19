use std::env;
use actix_web::{post, App, HttpRequest, HttpServer, Result};
use log::info;


#[post("/{topic}")]
async fn index(req: HttpRequest) -> Result<String> {
    // TODO: write to Kafka topic
    //let topic: String = req.match_info().get("topic").unwrap().parse().unwrap();
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