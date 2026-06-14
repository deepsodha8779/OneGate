use crate::app_state::AppState;
use crate::method::convention::{self, ErrorData};
use crate::method::methods::notice::NoticeMethods;
use crate::services::notice::command::{add_notice, delete_notice, update_notice};
use crate::services::notice::query::get_notice_data;
use crate::utils::biscuit::check_role::check_role_exist;
use crate::utils::biscuit::get_details::get_society_detail;
use actix_web::web;
use biscuit_auth::Biscuit;
use serde_json::Value;

pub async fn process(
    methods: NoticeMethods,
    state: &AppState,
    biscuit: web::ReqData<Biscuit>,
) -> Result<Value, convention::ErrorData> {
    if check_role_exist(
        vec!["admin".to_string(), "superadmin".to_string()],
        &biscuit,
    )
    .is_ok()
    {
        let id = get_society_detail(&biscuit);
        match id {
            Ok((society_id, _)) => match methods {
                NoticeMethods::Add(input, _) => {
                    add_notice(input, state.clone(), society_id, biscuit).await
                }
                NoticeMethods::Update(input, _) => {
                    update_notice(input.input, input.id, state.clone(), society_id, biscuit).await
                }
                NoticeMethods::Delete(input, _) => {
                    delete_notice(input.id, state.clone(), society_id, biscuit).await
                }
                NoticeMethods::GetAll => get_notice_data(state.clone(), society_id, biscuit).await,
                NoticeMethods::GetById(_, _) => todo!(),
            },
            Err(error) => Err(ErrorData {
                message: String::from(error.to_string()),
                data: Value::Null,
                code: -32600,
            }),
        }
    } else {
        Err(ErrorData {
            message: String::from("Authorization Error"),
            data: Value::Null,
            code: -32600,
        })
    }
}
