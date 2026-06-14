use serde_json::Value;

use crate::{app_state::AppState, method::convention::ErrorData};

pub async fn get_entry_data(_data: AppState) -> Result<Value, ErrorData> {
    serde_json::to_value(String::from("Successful")).map_err(ErrorData::from)
}
