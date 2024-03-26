use actix_web::{get, Result, web::Data, error};


use crate::app_state::{AppState, Status};


#[get("/health_check")]
pub async fn health_check(app_state: Data<AppState>) -> Result<String> {
    let error = match app_state.status {
        Status::Ok => return Ok(String::from("")),
        Status::Initializing => String::from("initializing"),
    };
    Err(error::ErrorInternalServerError(error))
}