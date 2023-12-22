use actix_web::{post, App, HttpRequest, HttpServer, Result, web::{Bytes, Data}};
use log::info;

use crate::app_state::AppState;
use crate::kafka_producer::KafkaProducer;
use crate::configuration::Config;

#[post("/{topic}")]
async fn index(req: HttpRequest, app_state: Data<AppState>, payload: Bytes) -> Result<String> {
    let topic: String = req.match_info().get("topic").unwrap().parse().unwrap();
    app_state.producer.produce(topic.as_str(), payload.to_vec()).await;
    Ok(String::from(""))
}

pub async fn start(cfg: Config) -> std::io::Result<()> {
    info!("Listening on {}:{}", cfg.http.host, cfg.http.port);
    HttpServer::new(move || {
        let producer = KafkaProducer::new(&cfg.kafka);

        App::new()
            .app_data(Data::new(AppState::new(producer)))
            .service(index)
    })
    .bind((cfg.http.host, cfg.http.port))?
    .run()
    .await
}