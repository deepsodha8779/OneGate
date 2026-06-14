use anyhow::Result;

use crate::{app_state::AppState, dto::owner::OwnerInput, method::convention::ErrorData};

use serde_json::Value;
use uuid::Uuid;

pub async fn add_owner(_input: OwnerInput, data: AppState) -> Result<Value, ErrorData> {
    let mut tr = data.pool.begin().await?;
    let insert_query = "INSERT INTO owner(id,name, floor_no,flat_name) VALUES($1,$2,$3,$4)";
    let _ = sqlx::query(insert_query)
        .bind(Uuid::new_v4().as_simple().to_string())
        .bind(&_input.name)
        .bind(&_input.floor_no)
        .bind(&_input.flat_no)
        .execute(tr.as_mut())
        .await?;
    tr.commit().await?;
    serde_json::to_value(String::from("Successful")).map_err(ErrorData::from)
}

pub async fn update_owner(
    input: OwnerInput,
    _id: String,
    data: AppState,
) -> Result<Value, ErrorData> {
    let transaction = data.pool.begin().await?;
    let query = "UPDATE owner SET
    name  =$1
    flat_name = $2,
    floor_no =$3,
    WHERE id = $4 RETURNING id";
    let _update_query: i32 = sqlx::query_scalar(query)
        .bind(input.name)
        .bind(input.floor_no)
        .bind(input.flat_no)
        .fetch_one(&data.pool)
        .await?;
    transaction.commit().await?;
    serde_json::to_value(String::from("Successful")).map_err(ErrorData::from)
}

pub async fn delete_owner(_id: String, _data: AppState) -> Result<Value, ErrorData> {
    serde_json::to_value(String::from("Successful")).map_err(ErrorData::from)
}
