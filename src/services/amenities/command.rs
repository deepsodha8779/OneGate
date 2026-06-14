use crate::{
    app_state::AppState, dto::amenity::AmenityInput, method::convention::ErrorData,
    utils::biscuit::check_role::check_role_exist,
};
use actix_web::web;
use anyhow::Result;

use biscuit_auth::Biscuit;
use chrono::Utc;
use serde_json::Value;
use uuid::Uuid;

pub async fn add_amenity(
    input: AmenityInput,
    data: AppState,
    society_id: String,
    biscuit: web::ReqData<Biscuit>,
) -> Result<Value, ErrorData> {
    if check_role_exist(
        vec!["admin".to_string(), "superadmin".to_string()],
        &biscuit,
    )
    .is_ok()
    {
        let mut tr = data.pool.begin().await?;
        let insert_query =
        "INSERT INTO  amenity (id,name,description, start_time ,end_time,society_id) VALUES($1,$2,$3,$4,$5,$6);";
        let _ = sqlx::query(insert_query)
            .bind(Uuid::new_v4().as_simple().to_string())
            .bind(&input.name)
            .bind(&input.description)
            .bind(Utc::now().to_string())
            .bind(Utc::now().to_string())
            .bind(society_id)
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

pub async fn update_amenity(
    input: AmenityInput,
    id: String,
    data: AppState,
    society_id: String,
    biscuit: web::ReqData<Biscuit>,
) -> Result<Value, ErrorData> {
    if check_role_exist(
        vec!["admin".to_string(), "superadmin".to_string()],
        &biscuit,
    )
    .is_ok()
    {
        let mut transaction = data.pool.begin().await?;

        let user_details_query = "UPDATE amenity SET name = $1, description = $2 ,start_time = $3 , end_time = $4 WHERE id = $5 AND society_id = $6";

        let _ = sqlx::query(user_details_query)
            .bind(input.name)
            .bind(input.description)
            .bind(input.start_time)
            .bind(input.end_time)
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

pub async fn delete_amenity(
    id: String,
    data: AppState,
    society_id: String,
    biscuit: web::ReqData<Biscuit>,
) -> Result<Value, ErrorData> {
    if check_role_exist(
        vec!["admin".to_string(), "superadmin".to_string()],
        &biscuit,
    )
    .is_ok()
    {
        let mut transaction = data.pool.begin().await?;
        let delete_query = "
        DELETE FROM amenity WHERE id = $1 AND society_id = $2  RETURNING id;
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
