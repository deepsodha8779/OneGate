use crate::{
    app_state::AppState, dto::swimming_pool::SwimmingPoolInput, method::convention::ErrorData,
};
use anyhow::Result;
use serde_json::Value;

pub async fn add_swimming_pool(
    _input: SwimmingPoolInput,
    _data: AppState,
    _society_id: String,
) -> Result<Value, ErrorData> {
    serde_json::to_value(String::from("Successful")).map_err(ErrorData::from)
}

pub async fn update_swimming_pool(
    _input: SwimmingPoolInput,
    _id: String,
    _data: AppState,
    _society_id: String,
) -> Result<Value, ErrorData> {
    serde_json::to_value(String::from("Successful")).map_err(ErrorData::from)
}

pub async fn delete_swimming_pool(
    _id: String,
    _data: AppState,
    _society_id: String,
) -> Result<Value, ErrorData> {
    serde_json::to_value(String::from("Successful")).map_err(ErrorData::from)
}
