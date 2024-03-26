pub mod endpoints;

use actix_web::{App, HttpServer, web::Data};
use log::info;

use crate::app_state::{AppState, Status};
use crate::kafka_producer::KafkaProducer;
use crate::configuration::Config;

use crate::http_api::endpoints::health_check::health_check;
use crate::http_api::endpoints::topics_post::topics_post;

pub async fn start(cfg: Config) -> std::io::Result<()> {
    info!("Listening on {}:{}", cfg.http.host, cfg.http.port);
    HttpServer::new(move || {
        // TODO: this creates a producer per Actix worker.
        // Should we have 1 producer instead? Should we pass Arc<Mutex<Producer>>?
        // Will it block HTTP handlers? Perhaps current approach is still better
        let producer = KafkaProducer::new(&cfg.kafka);
        let mut state = AppState::new(producer);
        state.status = Status::Ok; // FIXME: this has no effect. Need to catch conneciton issues other way

        App::new()
            .app_data(Data::new(state))
            .service(health_check)
            .service(topics_post)
    })
    .bind((cfg.http.host, cfg.http.port))?
    .run()
    .await
}