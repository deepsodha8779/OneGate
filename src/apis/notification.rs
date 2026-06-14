use crate::app_state::AppState;
use crate::method::convention;
use crate::method::methods::notification::NotificationMethods;
use crate::services::notification::command::{
    add_notification, delete_notification, update_notification,
};
use crate::services::notification::query::get_notification_data;
use serde_json::Value;

pub async fn process(
    methods: NotificationMethods,
    state: &AppState,
) -> Result<Value, convention::ErrorData> {
    match methods {
        NotificationMethods::Add(input, _) => add_notification(input, state.clone()).await,
        NotificationMethods::Update(input, _) => {
            update_notification(input.input, input.id, state.clone()).await
        }
        NotificationMethods::Delete(input, _) => delete_notification(input.id, state.clone()).await,
        NotificationMethods::GetAll => get_notification_data(state.clone()).await,
        NotificationMethods::GetById(_, _) => todo!(),
    }
}
