use serde_json::Value;
use uuid::Uuid;

use crate::{app_state::AppState, dto::owner::Owner, method::convention::ErrorData};

pub async fn get_owner_data(_data: AppState) -> Result<Value, ErrorData> {
    serde_json::to_value(vec![
        Owner {
            id: Uuid::new_v4().to_string(),
            name: "Mr. Sharma".to_string(),
            floor_no: "2".to_string(),
            flat_no: "2".to_string(),
        },
        Owner {
            id: Uuid::new_v4().to_string(),
            name: "Mr. Bansal".to_string(),
            floor_no: "5".to_string(),
            flat_no: "3".to_string(),
        },
        Owner {
            id: Uuid::new_v4().to_string(),
            name: "Mr. Singh".to_string(),
            floor_no: "8".to_string(),
            flat_no: "5".to_string(),
        },
    ])
    .map_err(ErrorData::from)
}
