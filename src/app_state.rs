use crate::kafka_producer::KafkaProducer;

#[derive(Clone)]
pub struct AppState {
    pub producer: KafkaProducer,
}

impl AppState {
    pub fn new(producer: KafkaProducer) -> Self {
        AppState {
            producer
        }
    }
}