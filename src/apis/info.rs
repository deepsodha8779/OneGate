use crate::apis::api_error::ApiError;
use actix_web::{get, web, HttpResponse};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS)]
#[ts(export, export_to = "bindings/Info.ts")]
struct Info {
    name: String,
    features: Vec<String>,
}

#[get("/{s_id}")]
pub async fn get_info(path: web::Path<String>) -> Result<HttpResponse, ApiError> {
    let s_id = path.into_inner();
    //TODO: query database
    //TODO: remove dumb logic of id verification
    if s_id.eq("1234") {
        Ok(HttpResponse::Ok().json(Info {
            features: vec![],
            name: "iLand".to_string(),
        }))
    } else {
        Err(ApiError::NotFound("Society Not Found".to_string()))
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_info);
}
