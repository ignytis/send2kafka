use actix_web::{post, App, HttpRequest, HttpResponse, HttpServer, Result,
    http::{header::ContentType, StatusCode},
    web::{Bytes, Data}, error};
use derive_more::Display;
use log::{error, info};

use crate::app_state::AppState;
use crate::kafka_producer::KafkaProducer;
use crate::configuration::Config;

#[derive(Debug, Display)]
struct KafkaError {}

impl error::ResponseError for KafkaError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
            .insert_header(ContentType::json())
            .body("")
    }
}   
   

#[post("/{topic}")]
async fn index(req: HttpRequest, app_state: Data<AppState>, payload: Bytes) -> Result<String, KafkaError> {
    let topic: String = req.match_info().get("topic").unwrap().parse().unwrap();
    match app_state.producer.produce(topic.as_str(), payload.to_vec()).await {
        Ok(_) => Ok(String::from("")),
        Err(e) => {
            error!("Failed to produce a Kafka message: {}", e);
            Err(KafkaError{})
        }
    }
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