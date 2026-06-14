use crate::app_state::AppState;
use crate::method::convention;
use crate::method::methods::owner::OwnerMethods;
use crate::services::owner::command::{add_owner, delete_owner, update_owner};
use crate::services::owner::query::get_owner_data;
use serde_json::Value;

pub async fn process(
    methods: OwnerMethods,
    state: &AppState,
) -> Result<Value, convention::ErrorData> {
    match methods {
        OwnerMethods::Add(input, _) => add_owner(input, state.clone()).await,
        OwnerMethods::Update(input, _) => update_owner(input.input, input.id, state.clone()).await,
        OwnerMethods::Delete(input, _) => delete_owner(input.id, state.clone()).await,
        OwnerMethods::GetAll => get_owner_data(state.clone()).await,
        OwnerMethods::GetById(_, _) => todo!(),
    }
}
