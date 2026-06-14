use crate::method::convention::{self, ErrorData};
use crate::method::methods::security::SecurityMethods;
use crate::services::security::command::{add_security, delete_security, update_security};
use crate::services::security::query::get_security_data;
use crate::utils::biscuit::get_details::get_society_detail;

use actix_web::web;
use biscuit_auth::Biscuit;
use serde_json::Value;

use crate::app_state::AppState;

pub async fn process(
    methods: SecurityMethods,
    state: &AppState,
    biscuit: web::ReqData<Biscuit>,
) -> Result<Value, convention::ErrorData> {
    let id = get_society_detail(&biscuit);
    match id {
        Ok((society_id, _)) => match methods {
            SecurityMethods::Add(input, _) => {
                add_security(input, state.clone(), society_id, biscuit).await
            }
            SecurityMethods::Update(input, _) => {
                update_security(input.input, input.id, state.clone(), society_id, biscuit).await
            }
            SecurityMethods::Delete(input, _) => {
                delete_security(input.id, state.clone(), society_id, biscuit).await
            }
            SecurityMethods::GetAll => get_security_data(state.clone(), society_id, biscuit).await,
            SecurityMethods::GetById(_, _) => todo!(),
        },
        Err(error) => Err(ErrorData {
            message: String::from(error.to_string()),
            data: Value::Null,
            code: -32600,
        }),
    }
}
