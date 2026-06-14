use crate::method::convention::{self, ErrorData};
use crate::method::methods::service_provider::ServiceProviderMethods;
use crate::services::service_provider::command::{
    add_or_create_service_provider, delete_service_provider, update_service_provider,
};
use crate::services::service_provider::query::get_service_providers;
use crate::utils::biscuit::get_details::get_society_detail;
use actix_web::web;
use biscuit_auth::Biscuit;
use serde_json::Value;

use crate::app_state::AppState;

pub async fn process(
    methods: ServiceProviderMethods,
    state: &AppState,
    biscuit: web::ReqData<Biscuit>,
) -> Result<Value, convention::ErrorData> {
    let id = get_society_detail(&biscuit);
    match id {
        Ok((society_id, _)) => match methods {
            ServiceProviderMethods::Add(input, _) => {
                add_or_create_service_provider(input, state.clone(), society_id, biscuit).await
            }
            ServiceProviderMethods::Update(input, _) => {
                update_service_provider(input.input, input.id, state.clone(), society_id, biscuit)
                    .await
            }
            ServiceProviderMethods::Delete(input, _) => {
                delete_service_provider(input.id, state.clone(), society_id, biscuit).await
            }
            ServiceProviderMethods::GetById(_, _) => todo!(),
            ServiceProviderMethods::GetAll => {
                get_service_providers(state.clone(), society_id, biscuit).await
            }
        },
        Err(error) => Err(ErrorData {
            message: String::from(error.to_string()),
            data: Value::Null,
            code: -32600,
        }),
    }
}
