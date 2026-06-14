use crate::method::convention;
use crate::method::methods::entry_log::EntryLogMethods;
use crate::services::entry_log::command::{add_entry, delete_entry, update_entry};
use crate::services::entry_log::query::get_entry_data;
use serde_json::Value;

use crate::app_state::AppState;

pub async fn process(
    methods: EntryLogMethods,
    state: &AppState,
) -> Result<Value, convention::ErrorData> {
    match methods {
        EntryLogMethods::Add(input, _) => add_entry(input, state.clone()).await,
        EntryLogMethods::Update(input, _) => {
            update_entry(input.input, input.id, state.clone()).await
        }
        EntryLogMethods::Delete(input, _) => delete_entry(input.id, state.clone()).await,
        EntryLogMethods::GetAll => get_entry_data(state.clone()).await,
        EntryLogMethods::GetById(_, _) => todo!(),
    }
}
