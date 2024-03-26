use crate::kafka_producer::KafkaProducer;

#[derive(Clone)]
pub enum Status {
    Initializing,
    Ok,
}

#[derive(Clone)]
pub struct AppState {
    pub producer: KafkaProducer,
    pub status: Status,
}

impl AppState {
    pub fn new(producer: KafkaProducer) -> Self {
        AppState{producer, status: Status::Initializing}
    }
}