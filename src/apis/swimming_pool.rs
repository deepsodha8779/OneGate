use crate::method::convention::{self, ErrorData};
use crate::method::methods::swimming_pool::SwimmingPoolMethods;
use crate::services::swimming_pool::command::{
    add_swimming_pool, delete_swimming_pool, update_swimming_pool,
};
use crate::services::swimming_pool::query::get_swimming_pool_data;
use crate::utils::biscuit::check_role::check_role_exist;
use crate::utils::biscuit::get_details::get_society_detail;
use actix_web::web;
use biscuit_auth::Biscuit;
use serde_json::Value;

use crate::app_state::AppState;

pub async fn process(
    methods: SwimmingPoolMethods,
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
                SwimmingPoolMethods::Add(input, _) => {
                    add_swimming_pool(input, state.clone(), society_id).await
                }
                SwimmingPoolMethods::Update(input, _) => {
                    update_swimming_pool(input.input, input.id, state.clone(), society_id).await
                }
                SwimmingPoolMethods::Delete(input, _) => {
                    delete_swimming_pool(input.id, state.clone(), society_id).await
                }
                SwimmingPoolMethods::GetAll => {
                    get_swimming_pool_data(state.clone(), society_id).await
                }
                SwimmingPoolMethods::GetById(_, _) => todo!(),
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
