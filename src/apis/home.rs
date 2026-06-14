use crate::method::convention::{self, ErrorData};
use crate::method::methods::home::HomeMethods;
use crate::services::home::command::{add_home_submit, delete_home, update_home};
use crate::services::home::query::get_home_data;
use crate::utils::biscuit::get_details::get_society_detail;
use actix_web::web;
use biscuit_auth::Biscuit;
use serde_json::Value;

use crate::app_state::AppState;

pub async fn process(
    methods: HomeMethods,
    state: &AppState,
    biscuit: web::ReqData<Biscuit>,
) -> Result<Value, convention::ErrorData> {
    let id = get_society_detail(&biscuit);
    match id {
        Ok((society_id, _)) => match methods {
            HomeMethods::Add(input, _) => {
                add_home_submit(input, state.clone(), society_id, biscuit).await
            }
            HomeMethods::Update(input, _) => {
                update_home(input.input, input.id, state.clone(), society_id, biscuit).await
            }
            HomeMethods::Delete(input, _) => {
                delete_home(input.id, state.clone(), society_id, biscuit).await
            }
            HomeMethods::GetAll => get_home_data(state.clone(), society_id, biscuit).await,
            HomeMethods::GetById(_, _) => todo!(),
        },
        Err(error) => Err(ErrorData {
            message: String::from(error.to_string()),
            data: Value::Null,
            code: -32600,
        }),
    }
}
