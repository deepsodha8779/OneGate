use crate::method::convention::{self};
use crate::method::methods::society::SocietyMethods;
use crate::services::society::{
    command::{create_society, delete_society, update_society},
    query::get_societies,
};
use actix_web::web;
use biscuit_auth::Biscuit;
use serde_json::Value;

use crate::app_state::AppState;

pub async fn process_society(
    methods: SocietyMethods,
    state: &AppState,
    biscuit: web::ReqData<Biscuit>,
) -> Result<Value, convention::ErrorData> {
    match methods {
        SocietyMethods::Add(input, _) => create_society(input, state.clone(), biscuit).await,
        SocietyMethods::Update(input, _) => {
            update_society(input.input, input.id, state.clone(), biscuit).await
        }
        SocietyMethods::Delete(input, _) => delete_society(input.id, state.clone(), biscuit).await,
        SocietyMethods::GetAll => get_societies(state.clone(), biscuit).await,
        SocietyMethods::GetById(_, _) => todo!(),
    }
}
