use crate::{
    app_state::AppState, database::complaint::Complaint, method::convention::ErrorData,
    utils::biscuit::check_role::check_role_exist,
};
use actix_web::web;
use biscuit_auth::Biscuit;
use serde_json::Value;

pub async fn get_complaint_data(
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
        let query =    "SELECT id, complaint_by, complaint_title , complaint_description , status FROM complaint WHERE society_id = $1";
        let rows = sqlx::query_as::<_, Complaint>(query)
            .bind(&society_id)
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
