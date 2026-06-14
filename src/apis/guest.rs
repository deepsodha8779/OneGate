use crate::app_state::AppState;
use crate::method::convention;
use crate::method::methods::guest::GuestMethods;
use crate::services::guest::command::{add_guest, delete_guest, update_guest};
use crate::services::guest::query::get_guest_data;
use serde_json::Value;

pub async fn process(
    methods: GuestMethods,
    state: &AppState,
) -> Result<Value, convention::ErrorData> {
    match methods {
        GuestMethods::Add(input, _) => add_guest(input, state.clone()).await,
        GuestMethods::Update(input, _) => update_guest(input.input, input.id, state.clone()).await,
        GuestMethods::Delete(input, _) => delete_guest(input.id, state.clone()).await,
        GuestMethods::GetAll => get_guest_data(state.clone()).await,
        GuestMethods::GetById(_, _) => todo!(),
    }
}
