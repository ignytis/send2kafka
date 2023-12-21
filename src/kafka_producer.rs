use std::time::Duration;

use log::info;

use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};

pub struct KafkaProducer {
    rd_producer: FutureProducer,
}

impl KafkaProducer {
    pub fn new(kafka_bootstrap_servers: String) -> Self {
        let rd_producer = ClientConfig::new()
            .set("bootstrap.servers", kafka_bootstrap_servers)
            .set("message.timeout.ms", "5000")
            .create()
            .expect("Can't create a Kafka producer");

        KafkaProducer {
            rd_producer
        }
    }

    pub async fn produce(&self, topic_name: &str, payload: Vec<u8>) {
        let res = self.rd_producer
            .send(FutureRecord::to(topic_name)
                    .payload(&payload.to_vec())
                    .key(&String::new()),
                Duration::from_secs(0)).await;
        info!("Result: {:?}", res);
    }
}