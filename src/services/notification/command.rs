use anyhow::Result;

use crate::{
    app_state::AppState, dto::notification::NotificationInput, method::convention::ErrorData,
};

use serde_json::Value;
use uuid::Uuid;

pub async fn add_notification(
    input: NotificationInput,
    data: AppState,
) -> Result<Value, ErrorData> {
    let mut tr = data.pool.begin().await?;
    let insert_query = "INSERT INTO notification(id,heading,description) VALUES($1,$2,$3)";
    let _ = sqlx::query(insert_query)
        .bind(Uuid::new_v4().as_simple().to_string())
        .bind(&input.heading)
        .bind(&input.description)
        .execute(tr.as_mut())
        .await?;
    tr.commit().await?;
    serde_json::to_value(String::from("Successful")).map_err(ErrorData::from)
}

pub async fn update_notification(
    input: NotificationInput,
    _id: String,
    data: AppState,
) -> Result<Value, ErrorData> {
    let transaction = data.pool.begin().await?;
    let query = "UPDATE notification SET
    heading = $1,
    description =$2,
    WHERE id = $3 RETURNING id";
    let _update_query: i32 = sqlx::query_scalar(query)
        .bind(input.heading)
        .bind(input.description)
        .fetch_one(&data.pool)
        .await?;
    transaction.commit().await?;
    serde_json::to_value(String::from("Successful")).map_err(ErrorData::from)
}

pub async fn delete_notification(id: String, data: AppState) -> Result<Value, ErrorData> {
    let transaction = data.pool.begin().await?;
    let query = "DELETE FROM notification WHERE id = $1 RETURNING id";
    let _insert_query: i32 = sqlx::query_scalar(query)
        .bind(id)
        .fetch_one(&data.pool)
        .await?;
    transaction.commit().await?;
    serde_json::to_value(String::from("Successful")).map_err(ErrorData::from)
}
