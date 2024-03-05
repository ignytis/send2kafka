use std::collections::HashMap;
use std::time::Duration;

use log::info;
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
        let rd_producer = rd_producer.create().expect("Can't create a Kafka producer");
        let bootstrap_servers = cfg.get("bootstrap_servers")
            .unwrap_or(&String::from("(not provided)"))
            .clone();  // to resolve a '&String vs String' issue
        info!("Connected to Kafka. Bootstrap servers: {}", bootstrap_servers);

        KafkaProducer { rd_producer }
    }

    // On success returns a tuple (partition, offset)
    // On failure returns an error message
    pub async fn produce(&self, topic_name: &str, key: &Option<String>, payload: Vec<u8>) -> Result<(i32, i64), String> {
        let p = &payload.to_vec();
        let mut future = FutureRecord::to(topic_name) .payload(p);
        future = match key {
            Some(k) => future.key(k),
            None => future,
        };
        let res = self.rd_producer
            .send(future, Duration::from_secs(0)).await;
        match res {
            Ok(d) => Ok(d),
            Err((kafka_error, _)) => Err(format!("{:?}", kafka_error))
        }
    }
}