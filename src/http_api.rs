use actix_web::{post, App, HttpRequest, HttpResponse, HttpServer, Result,
    http::StatusCode,
    web::{Bytes, Data, Path}, error};
use derive_more::Display;
use log::{error, info};

use crate::app_state::AppState;
use crate::kafka_producer::KafkaProducer;
use crate::configuration::Config;

const ERR_PRODUCE_KAFKA_MESSAGE: &str = "Failed to produce a Kafka message";

#[derive(Debug, Display)]
enum TopicPostErrorKind {
    /// Produicer failed to send a message to Kafka
    ProducerError,
}

#[derive(Debug, Display)]
struct TopicPostError {
    kind: TopicPostErrorKind,
}

impl TopicPostError {
    fn new(kind: TopicPostErrorKind) -> Self {
        TopicPostError { kind }
    }
}

impl error::ResponseError for TopicPostError {
    fn error_response(&self) -> HttpResponse {
        let (body, status_code) = match self.kind {
            TopicPostErrorKind::ProducerError => (ERR_PRODUCE_KAFKA_MESSAGE, StatusCode::INTERNAL_SERVER_ERROR),
        };
        HttpResponse::build(status_code).body(body)
    }
}   
   

#[post("/topics/{topic}")]
async fn index(req: HttpRequest, path_info: Path<String>, payload: Bytes, app_state: Data<AppState>) -> Result<String, TopicPostError> {
    let topic: String = path_info.into_inner();
    // Probably could replace this validation logic with Actix guard, but in this case we will not get error details
    let key = match req.headers().get("X-Key") {
        Some(h) => match h.to_str() {
            Ok(v) => Some(String::from(v)),
            Err(_) => None,
        },
        None => None,
    };
    match app_state.producer.produce(topic.as_str(), &key, payload.to_vec()).await {
        Ok(_) => Ok(String::from("")),
        Err(e) => {
            error!("{}: {}", ERR_PRODUCE_KAFKA_MESSAGE, e);
            Err(TopicPostError::new(TopicPostErrorKind::ProducerError))
        }
    }
}

pub async fn start(cfg: Config) -> std::io::Result<()> {
    info!("Listening on {}:{}", cfg.http.host, cfg.http.port);
    HttpServer::new(move || {
        // TODO: this creates a producer per Actix worker.
        // Should we have 1 producer instead? Should we pass Arc<Mutex<Producer>>?
        // Will it block HTTP handlers? Perhaps current approach is still better
        let producer = KafkaProducer::new(&cfg.kafka);

        App::new()
            .app_data(Data::new(AppState::new(producer)))
            .service(index)
    })
    .bind((cfg.http.host, cfg.http.port))?
    .run()
    .await
}