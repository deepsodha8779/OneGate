use crate::method::convention;
use crate::method::methods::staff::StaffMethods;
use crate::services::staff::command::{add_staff, delete_staff, update_staff};
use crate::services::staff::query::get_staff_data;
use serde_json::Value;

use crate::app_state::AppState;

pub async fn process(
    methods: StaffMethods,
    state: &AppState,
) -> Result<Value, convention::ErrorData> {
    match methods {
        StaffMethods::Add(input, _) => add_staff(input, state.clone()).await,
        StaffMethods::Update(input, _) => update_staff(input.input, input.id, state.clone()).await,
        StaffMethods::Delete(input, _) => delete_staff(input.id, state.clone()).await,
        StaffMethods::GetAll => get_staff_data(state.clone()).await,
        StaffMethods::GetById(_, _) => todo!(),
    }
}
