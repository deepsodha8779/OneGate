use serde_json::Value;
use uuid::Uuid;

use crate::{app_state::AppState, dto::notification::Notification, method::convention::ErrorData};

pub async fn get_notification_data(_data: AppState) -> Result<Value, ErrorData> {
    serde_json::to_value(vec![
        Notification {
            id: Uuid::new_v4().to_string(),
            heading: "Notification".to_string(),
            description: "Society has a fire drill arranged at 2:00 pm. It is requested to everyone to kindly attend.".to_string(),
        },
        Notification {
            id: Uuid::new_v4().to_string(),
            heading: "Notification".to_string(),
            description:
                "Swimming pool has a booking for Afternoon slot.".to_string(),
        },
        Notification {
            id: Uuid::new_v4().to_string(),
            heading: "Notification".to_string(),
            description:
                "Monthly Collection due on 24th of this month."
                    .to_string(),
        },
    ])
    .map_err(ErrorData::from)
}
