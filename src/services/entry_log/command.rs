use crate::{app_state::AppState, dto::entrylog::EntryLogInput, method::convention::ErrorData};
use anyhow::Result;
use serde_json::Value;
use uuid::Uuid;

pub async fn add_entry(input: EntryLogInput, data: AppState) -> Result<Value, ErrorData> {
    let mut tr = data.pool.begin().await?;

    let insert_query = "INSERT INTO entrylog(id,user_id,enter_date_time, exit_date_time)
         VALUES($1,$2,$3) where id =1";
    let _ = sqlx::query(insert_query)
        .bind(Uuid::new_v4().as_simple().to_string())
        .bind(input.enter_date_time)
        .bind(input.user.is_block)
        .bind(input.exit_date_time)
        .execute(tr.as_mut())
        .await?;
    tr.commit().await?;

    serde_json::to_value(String::from("Successful")).map_err(ErrorData::from)
}

pub async fn update_entry(
    input: EntryLogInput,
    _id: String,
    data: AppState,
) -> Result<Value, ErrorData> {
    let transaction = data.pool.begin().await?;
    let query = "UPDATE complaint SET
    user = $1,
    enter_date_time = $2,
    exit_date_time =$3,
    WHERE id = $3 RETURNING id";
    let _update_query: i32 = sqlx::query_scalar(query)
        .bind(input.enter_date_time)
        .bind(input.exit_date_time)
        .bind(input.user.is_block)
        .fetch_one(&data.pool)
        .await?;
    transaction.commit().await?;
    serde_json::to_value(String::from("Successful")).map_err(ErrorData::from)
}

pub async fn delete_entry(id: String, data: AppState) -> Result<Value, ErrorData> {
    let transaction = data.pool.begin().await?;
    let query = "DELETE FROM entrylog WHERE id = $1 RETURNING id";
    let _insert_query: i32 = sqlx::query_scalar(query)
        .bind(id)
        .fetch_one(&data.pool)
        .await?;
    transaction.commit().await?;
    serde_json::to_value(String::from("Successful")).map_err(ErrorData::from)
}
