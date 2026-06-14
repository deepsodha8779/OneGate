use crate::{
    app_state::AppState, dto::home_types::HomeInput, method::convention::ErrorData,
    utils::biscuit::check_role::check_role_exist,
};
use actix_web::web;
use biscuit_auth::Biscuit;
use serde_json::Value;
use uuid::Uuid;

pub async fn add_home_submit(
    input: HomeInput,
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
        let mut tr = data.pool.begin().await?;
        let insert_query = "INSERT INTO home (id,society_id, wing,no)
         VALUES($1,$2,$3,$4)";
        let _ = sqlx::query(insert_query)
            .bind(Uuid::new_v4().as_simple().to_string())
            .bind(society_id)
            .bind(input.wing)
            .bind(input.no)
            .execute(tr.as_mut())
            .await?;
        tr.commit().await?;
        serde_json::to_value(String::from("Successful")).map_err(ErrorData::from)
    } else {
        Err(ErrorData {
            message: String::from("Authorization Error"),
            data: Value::Null,
            code: -32600,
        })
    }
}

pub async fn update_home(
    input: HomeInput,
    id: String,
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
        let mut transaction = data.pool.begin().await?;

        let user_details_query =
            "UPDATE home SET wing = $1, no = $2 WHERE id = $3 AND society_id = $4";

        let _ = sqlx::query(user_details_query)
            .bind(input.wing)
            .bind(input.no)
            .bind(id)
            .bind(society_id)
            .execute(transaction.as_mut())
            .await?;
        transaction.commit().await?;
        serde_json::to_value(String::from("Successful")).map_err(ErrorData::from)
    } else {
        Err(ErrorData {
            message: String::from("Authorization Error"),
            data: Value::Null,
            code: -32600,
        })
    }
}

pub async fn delete_home(
    id: String,
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
        let mut transaction = data.pool.begin().await?;
        let delete_query = "
        DELETE FROM home WHERE id = $1 AND society_id = $2  RETURNING id;
    ";

        let deleted_id: String = sqlx::query_scalar(delete_query)
            .bind(id.clone())
            .bind(society_id)
            .fetch_one(transaction.as_mut())
            .await?;

        transaction.commit().await?;

        serde_json::to_value(deleted_id).map_err(ErrorData::from)
    } else {
        Err(ErrorData {
            message: String::from("Authorization Error"),
            data: Value::Null,
            code: -32600,
        })
    }
}
