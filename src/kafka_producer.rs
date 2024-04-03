use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use log::{error, info};
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};

use lazy_static::lazy_static;


pub struct Stats {
    pub num_errors: usize,
}

lazy_static! {
    pub static ref STATS: Arc<Mutex<Stats>> = Arc::new(Mutex::new(Stats {
        num_errors: 0,
    }));
}

fn queue_poll_error_cb(message: String) {
    STATS.lock().unwrap().num_errors += 1;
    error!("Error callback CLOSURE: {}; ERRORS REPORTED: {}", message, STATS.lock().unwrap().num_errors);
}

#[derive(Clone)]
pub struct KafkaProducer {
    rd_producer: FutureProducer,
}


impl KafkaProducer {
    pub fn new(cfg: &HashMap<String, String>) -> Self {
        let bootstrap_servers = cfg.get("bootstrap_servers")
            .unwrap_or(&String::from("(not provided)"))
            .clone();  // to resolve a '&String vs String' issue
        info!("Bootstrap servers: {}", bootstrap_servers);

        let mut kafka_config = &mut ClientConfig::new();
        for (k, v) in cfg.iter() {
            kafka_config = kafka_config.set(k.replace("_", "."), v);
        }
        kafka_config = kafka_config.set_queue_poll_error_cb(queue_poll_error_cb);

        KafkaProducer {
            rd_producer: kafka_config.create().expect("Can't create a Kafka producer")
        }
    }

    // On success returns a tuple (partition, offset)
    // On failure returns an error message
    pub async fn produce(&self, topic_name: &str, key: &Option<String>, payload: Vec<u8>) -> Result<(i32, i64), String> {
        let p = &payload.to_vec();
        let mut future_record = FutureRecord::to(topic_name).payload(p);
        future_record = match key {
            Some(k) => future_record.key(k),
            None => future_record,
        };
        let res = self.rd_producer
            .send(future_record, Duration::from_secs(0)).await;

        match res {
            Ok(d) => Ok(d),
            Err((kafka_error, _)) => Err(format!("{:?}", kafka_error))
        }
    }
}