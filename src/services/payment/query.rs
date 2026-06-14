use serde_json::Value;
use uuid::Uuid;

use crate::{app_state::AppState, dto::payment::Payment, method::convention::ErrorData};

pub async fn get_payment_data(_data: AppState) -> Result<Value, ErrorData> {
    serde_json::to_value(vec![
        Payment {
            id: Uuid::new_v4().to_string(),
            name: "MARK".to_string(),
            plan: "2014".to_string(),
        },
        Payment {
            id: Uuid::new_v4().to_string(),
            name: "Evan".to_string(),
            plan: "2013".to_string(),
        },
        Payment {
            id: Uuid::new_v4().to_string(),
            name: "Onam".to_string(),
            plan: "2014".to_string(),
        },
        Payment {
            id: Uuid::new_v4().to_string(),
            name: "Daisy".to_string(),
            plan: "2011".to_string(),
        },
    ])
    .map_err(ErrorData::from)
}
