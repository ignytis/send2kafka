use std::collections::HashMap;
use std::time::Duration;

use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};

#[derive(Clone)]
pub struct KafkaProducer {
    rd_producer: FutureProducer,
}

impl KafkaProducer {
    pub fn new(cfg: &HashMap<String, String>) -> Self {
        let mut rd_producer = &mut ClientConfig::new();
        for (k, v) in cfg.iter() {
            rd_producer = rd_producer.set(k.replace("_", "."), v);   
        }

        KafkaProducer {
            rd_producer: rd_producer.create().expect("Can't create a Kafka producer"),
        }
    }

    // On success returns a tuple (partition, offset)
    // On failure returns an error message
    pub async fn produce(&self, topic_name: &str, key: &String, payload: Vec<u8>) -> Result<(i32, i64), String> {
        let res = self.rd_producer
            .send(FutureRecord::to(topic_name)
                    .payload(&payload.to_vec())
                    .key(key), // TODO: format key
                Duration::from_secs(0)).await;
        match res {
            Ok(d) => Ok(d),
            Err((kafka_error, _)) => Err(format!("{:?}", kafka_error))
        }
    }
}