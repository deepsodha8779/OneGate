use crate::{
    app_state::AppState, database::service_provider::ServiceProviderData,
    method::convention::ErrorData, utils::biscuit::check_role::check_role_exist,
};
use actix_web::web;
use biscuit_auth::Biscuit;
use serde_json::Value;

pub async fn get_service_providers(
    data: AppState,
    society_id: String,
    biscuit: web::ReqData<Biscuit>,
) -> Result<Value, ErrorData> {
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
        let query = r#"
        SELECT
            service_provider.id,
            service_provider.service_provider_types,
            users.id as user_id,
            users.is_block,
            users.is_deleted,
            user_details.id as user_detail_id, -- Alias without quotes
            user_details.first_name,
            user_details.last_name,
            user_details.aadhar_number,
            user_details.photo_url,
            user_details.contact_number,
            user_details.contact_type,
            user_details.email
        FROM service_provider
        INNER JOIN users  ON service_provider.user_id = users.id
        INNER JOIN user_details ON users.user_detail_id = user_details.id
        WHERE users.society_id = $1
    "#;

        let service_providers = sqlx::query_as::<_, ServiceProviderData>(query)
            .bind(society_id)
            .fetch_all(&data.pool)
            .await
            .map_err(|err| ErrorData::from(err))?;

        serde_json::to_value(service_providers).map_err(ErrorData::from)
    } else {
        Err(ErrorData {
            message: String::from("Authorization Error"),
            data: Value::Null,
            code: -32600,
        })
    }
}
