use actix_web::{post, HttpRequest, HttpResponse, Result,
    http::StatusCode,
    web::{Bytes, Data, Path}, error};
use derive_more::Display;
use log::error;


use crate::app_state::AppState;

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
pub async fn topics_post(req: HttpRequest, path_info: Path<String>, payload: Bytes, app_state: Data<AppState>) -> Result<String, TopicPostError> {
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