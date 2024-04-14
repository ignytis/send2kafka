pub mod endpoints;

use actix_web::{App, HttpServer, web::Data};
use log::info;

use crate::app_state::AppState;
use crate::kafka_producer::KafkaProducer;
use crate::configuration::Config;

use crate::http_api::endpoints::topics_post::topics_post;

pub async fn start(cfg: Config) -> std::io::Result<()> {
    info!("Listening on {}:{}", cfg.http.host, cfg.http.port);
    let mut srv = HttpServer::new(move || {
        // TODO: this creates a producer per Actix worker.
        // Should we have 1 producer instead? Should we pass Arc<Mutex<Producer>>?
        // Will it block HTTP handlers? Perhaps current approach is still better
        let producer = KafkaProducer::new(&cfg.kafka);
        let state = AppState::new(producer);

        App::new()
            .app_data(Data::new(state))
            .service(topics_post)
    });

    if cfg.http.num_workers > 0 {
        srv = srv.workers(cfg.http.num_workers)
    }

    srv.bind((cfg.http.host, cfg.http.port))?
        .run()
        .await
}