use crate::method::convention::{self, ErrorData};
use crate::method::methods::user::UserMethods;
use crate::services::user::command::{add_user_submit, delete_user, update_user};
use crate::services::user::query::get_user_data;
use crate::utils::biscuit::check_role::check_role_exist;
use crate::utils::biscuit::get_details::get_society_detail;
use actix_web::web;
use biscuit_auth::Biscuit;
use serde_json::Value;

use crate::app_state::AppState;

pub async fn process(
    methods: UserMethods,
    state: &AppState,
    biscuit: web::ReqData<Biscuit>,
) -> Result<Value, convention::ErrorData> {
    if check_role_exist(
        vec![
            "admin".to_string(),
            "superadmin".to_string(),
            "user".to_string(),
        ],
        &biscuit,
    )
    .is_ok()
    {
        let id = get_society_detail(&biscuit);
        match id {
            Ok((society_id, _)) => match methods {
                UserMethods::Add(input, _) => {
                    add_user_submit(input, state.clone(), society_id, biscuit).await
                }
                UserMethods::Update(input, _) => {
                    update_user(input, state.clone(), society_id, biscuit).await
                }
                UserMethods::Delete(input, _) => {
                    delete_user(input.id, state.clone(), society_id, biscuit).await
                }
                UserMethods::GetAll => get_user_data(state.clone(), society_id, biscuit).await,
                UserMethods::GetById(_, _) => todo!(),
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
