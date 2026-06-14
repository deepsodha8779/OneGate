use actix_web::web;
use biscuit_auth::Biscuit;
use serde_json::Value;

use crate::{
    app_state::AppState, method::convention::ErrorData,
    utils::biscuit::check_role::check_role_exist, database::society::SocietyData,
};

pub async fn get_societies(
    data: AppState,
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
        SELECT * FROM society;
    "#;

        let society: Vec<SocietyData> = sqlx::query_as::<_, SocietyData>(query)
            .fetch_all(&data.pool)
            .await
            .map_err(|err| ErrorData::from(err))?;

        serde_json::to_value(society).map_err(ErrorData::from)
    } else {
        Err(ErrorData {
            message: String::from("Authorization Error"),
            data: Value::Null,
            code: -32600,
        })
    }
}
