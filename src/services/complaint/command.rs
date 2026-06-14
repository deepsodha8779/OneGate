use crate::{
    app_state::AppState, dto::complaint::ComplaintInput, method::convention::ErrorData,
    utils::biscuit::check_role::check_role_exist,
};
use actix_web::web;
use anyhow::Result;

use biscuit_auth::Biscuit;
use serde_json::Value;

pub async fn add_complaint(
    input: ComplaintInput,
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

        let complaint_query =
            "INSERT INTO complaint (id,complaint_by,society_id,complaint_title,complaint_description,status)
            VALUES($1,$2,$3,$4,$5,$6)";

        let _ = sqlx::query(complaint_query)
            .bind(uuid::Uuid::new_v4().as_simple().to_string())
            .bind(input.complaint_by)
            .bind(society_id)
            .bind(input.complaint_title)
            .bind(input.complaint_description)
            .bind(input.status)
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

pub async fn update_complaint(
    input: ComplaintInput,
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

        let user_details_query = "UPDATE complaint SET complaint_by = $1, complaint_title = $2 ,complaint_description = $3 , status = $4 WHERE id = $5 AND society_id = $6";

        let _ = sqlx::query(user_details_query)
            .bind(input.complaint_by)
            .bind(input.complaint_title)
            .bind(input.complaint_description)
            .bind(input.status)
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

pub async fn delete_complaint(
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
        DELETE FROM complaint WHERE id = $1 AND society_id = $2  RETURNING id;
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
