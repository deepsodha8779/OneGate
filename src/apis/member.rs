use crate::method::convention;
use crate::method::methods::member::MemberMethods;
use crate::services::member::command::{add_member, delete_member, update_member};
use crate::services::member::query::get_member_data;
use serde_json::Value;

use crate::app_state::AppState;

pub async fn process(
    methods: MemberMethods,
    state: &AppState,
) -> Result<Value, convention::ErrorData> {
    match methods {
        MemberMethods::Add(input, _) => add_member(input, state.clone()).await,
        MemberMethods::Update(input, _) => {
            update_member(input.input, input.id, state.clone()).await
        }
        MemberMethods::Delete(input, _) => delete_member(input.id, state.clone()).await,
        MemberMethods::GetAll => get_member_data(state.clone()).await,
        MemberMethods::GetById(_, _) => todo!(),
    }
}
