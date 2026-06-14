use crate::app_state::AppState;
use crate::method::convention::{self, ErrorData};
use crate::method::methods::complaint::ComplaintMethods;
use crate::services::complaint::command::{add_complaint, delete_complaint, update_complaint};
use crate::services::complaint::query::get_complaint_data;
use crate::utils::biscuit::get_details::get_society_detail;
use actix_web::web;
use biscuit_auth::Biscuit;
use serde_json::Value;

pub async fn process(
    methods: ComplaintMethods,
    state: &AppState,
    biscuit: web::ReqData<Biscuit>,
) -> Result<Value, convention::ErrorData> {
    let id = get_society_detail(&biscuit);
    match id {
        Ok((society_id, _)) => match methods {
            ComplaintMethods::Add(input, _) => {
                add_complaint(input, state.clone(), society_id, biscuit).await
            }
            ComplaintMethods::Update(input, _) => {
                update_complaint(input.input, input.id, state.clone(), society_id, biscuit).await
            }
            ComplaintMethods::Delete(input, _) => {
                delete_complaint(input.id, state.clone(), society_id, biscuit).await
            }
            ComplaintMethods::GetAll => {
                get_complaint_data(state.clone(), society_id, biscuit).await
            }
            ComplaintMethods::GetById(_, _) => todo!(),
        },
        Err(error) => Err(ErrorData {
            message: String::from(error.to_string()),
            data: Value::Null,
            code: -32600,
        }),
    }
}
