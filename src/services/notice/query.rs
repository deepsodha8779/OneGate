use actix_web::web;
use anyhow::Result;
use biscuit_auth::Biscuit;

use crate::{
    app_state::AppState, database::notice::Notice, method::convention::ErrorData,
    utils::biscuit::check_role::check_role_exist,
};

use serde_json::Value;

pub async fn get_notice_data(
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
        let query = "SELECT id, title, notice_description FROM notice WHERE society_id = $1";
        let rows = sqlx::query_as::<_, Notice>(query)
            .bind(society_id)
            .fetch_all(&data.pool)
            .await?;

        serde_json::to_value(rows).map_err(ErrorData::from)
    } else {
        Err(ErrorData {
            message: String::from("Authorization Error"),
            data: Value::Null,
            code: -32600,
        })
    }
}
