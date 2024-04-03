use actix_web::{get, Result, web::Data, error};


use crate::app_state::{AppState, Status};

use crate::kafka_producer::STATS;

#[get("/health_check")]
pub async fn health_check(app_state: Data<AppState>) -> Result<String> {
    let errors_num = STATS.lock().unwrap().num_errors;
    if errors_num > 0 {
        return Err(error::ErrorInternalServerError(String::from("Kafka error")));
    }

    let error = match app_state.status {
        Status::Ok => return Ok(String::from("")),
        Status::Initializing => String::from("initializing"),
    };
    Err(error::ErrorInternalServerError(error))
}