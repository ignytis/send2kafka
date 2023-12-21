use actix_web::{post, App, HttpRequest, HttpServer, Result, web::{Bytes, Data}};
use log::info;

use crate::app_state::AppState;
use crate::kafka_producer::KafkaProducer;

#[post("/{topic}")]
async fn index(req: HttpRequest, app_state: Data<AppState>, payload: Bytes) -> Result<String> {
    let topic: String = req.match_info().get("topic").unwrap().parse().unwrap();
    app_state.producer.produce(topic.as_str(), payload.to_vec()).await;
    Ok(String::from(""))
}

pub async fn start(http_host: String, http_port: u16, kafka_bootstrap_servers: String) -> std::io::Result<()> {
    info!("Listening on {}:{}", http_host, http_port);
    HttpServer::new(move || {
        let producer = KafkaProducer::new(kafka_bootstrap_servers.clone());

        App::new()
            .app_data(Data::new(AppState::new(producer)))
            .service(index)
    })
    .bind((http_host, http_port))?
    .run()
    .await
}