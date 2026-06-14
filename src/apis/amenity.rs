use crate::app_state::AppState;
use crate::method::convention::{self, ErrorData};
use crate::method::methods::amenity::AmenityMethods;
use crate::services::amenities::command::{add_amenity, delete_amenity, update_amenity};
use crate::services::amenities::query::get_amenity_data;
use crate::utils::biscuit::get_details::get_society_detail;

use actix_web::web;
use biscuit_auth::Biscuit;
use serde_json::Value;

pub async fn process(
    methods: AmenityMethods,
    state: &AppState,
    biscuit: web::ReqData<Biscuit>,
) -> Result<Value, convention::ErrorData> {
    let id = get_society_detail(&biscuit);
    match id {
        Ok((society_id, _)) => match methods {
            AmenityMethods::Add(input, _) => {
                add_amenity(input, state.clone(), society_id, biscuit).await
            }
            AmenityMethods::Update(input, _) => {
                update_amenity(input.input, input.id, state.clone(), society_id, biscuit).await
            }
            AmenityMethods::Delete(input, _) => {
                delete_amenity(input.id, state.clone(), society_id, biscuit).await
            }
            AmenityMethods::GetAll => get_amenity_data(state.clone(), society_id, biscuit).await,
        },
        Err(error) => Err(ErrorData {
            message: String::from(error.to_string()),
            data: Value::Null,
            code: -32600,
        }),
    }
}
