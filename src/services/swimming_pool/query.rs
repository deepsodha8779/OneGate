use serde_json::Value;
use uuid::Uuid;

use crate::{app_state::AppState, dto::swimming_pool::SwimmingPool, method::convention::ErrorData};

pub async fn get_swimming_pool_data(
    _data: AppState,
    _society_id: String,
) -> Result<Value, ErrorData> {
    serde_json::to_value(vec![
        SwimmingPool {
            id: Uuid::new_v4().to_string(),
            pool_type: "Small".to_string(),
            pool_details: " 10×20×3 feet".to_string(),
            rules: "People above age 10 not allowed".to_string(),
        },
        SwimmingPool {
            id: Uuid::new_v4().to_string(),
            pool_type: "Medium".to_string(),
            pool_details: "15×15×5feet".to_string(),
            rules: "People above age 10 and below 18 only allowed".to_string(),
        },
        SwimmingPool {
            id: Uuid::new_v4().to_string(),
            pool_type: "Big".to_string(),
            pool_details: "20×20×6feet".to_string(),
            rules: "People above age 18 allowed".to_string(),
        },
    ])
    .map_err(ErrorData::from)
}
