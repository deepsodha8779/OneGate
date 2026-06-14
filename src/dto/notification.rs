use serde::{Deserialize, Serialize};
// use validator::Validate;
use ts_rs::TS;
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq, Eq, TS)]
#[ts(export, export_to = "bindings/notification/Notification.ts ")]
pub struct Notification {
    pub id: String,
    pub heading: String,
    pub description: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq, Eq, TS)]
#[ts(export, export_to = "bindings/NotificationInput.ts")]
pub struct NotificationInput {
    pub heading: String,
    pub description: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, PartialEq, Eq, TS)]
#[ts(export, export_to = "bindings/notification/UpdateNotification.ts")]
pub struct UpdateNotification {
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub id: String,
    pub input: NotificationInput,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, PartialEq, Eq, TS)]
#[ts(export, export_to = "bindings/notification/Id.ts")]
pub struct Id {
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub id: String,
}
