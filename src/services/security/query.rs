use actix_web::web;
use biscuit_auth::Biscuit;
use serde_json::Value;

use crate::{
    app_state::AppState, database::security::SecurityData, method::convention::ErrorData,
    utils::biscuit::check_role::check_role_exist,
};

pub async fn get_security_data(
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
        let query1 = r#"
        SELECT
            security_details.id,
            security_details.start_time,
            security_details.end_time,
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
        FROM security_details
        INNER JOIN users  ON security_details.user_id = users.id
        INNER JOIN user_details ON users.user_detail_id = user_details.id
        WHERE users.society_id = $1
    "#;

        let security = sqlx::query_as::<_, SecurityData>(query1)
            .bind(society_id)
            .fetch_all(&data.pool)
            .await
            .map_err(|err| ErrorData::from(err))?;

        println!("security {:?}", security);

        serde_json::to_value(security).map_err(ErrorData::from)
    } else {
        Err(ErrorData {
            message: String::from("Authorization Error"),
            data: Value::Null,
            code: -32600,
        })
    }
}
