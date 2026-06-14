use crate::{app_state::AppState, dto::staff::StaffInput, method::convention::ErrorData};
use anyhow::Result;
use serde_json::Value;
use uuid::Uuid;

pub async fn add_staff(input: StaffInput, data: AppState) -> Result<Value, ErrorData> {
    let mut tr = data.pool.begin().await?;
    let insert_query =
        "INSERT INTO staff(id,full_name,contact,adharcard_no,photo) VALUES($1,$2,$3,$4,$5)";
    let _ = sqlx::query(insert_query)
        .bind(Uuid::new_v4().as_simple().to_string())
        .bind(input.full_name)
        .bind(input.contact)
        .bind(input.adharcard_no)
        .bind(input.photo)
        .execute(tr.as_mut())
        .await?;
    tr.commit().await?;
    serde_json::to_value(String::from("Successful")).map_err(ErrorData::from)
}

pub async fn update_staff(
    input: StaffInput,
    _id: String,
    data: AppState,
) -> Result<Value, ErrorData> {
    let transaction = data.pool.begin().await?;
    let query = "UPDATE staff SET 
    full_name = $1,
    contact = $2, 
    adharcard_no = $3,
    photo = $4,
    WHERE id = $5 RETURNING id";
    let _update_query: i32 = sqlx::query_scalar(query)
        .bind(Uuid::new_v4().as_simple().to_string())
        .bind(input.full_name)
        .bind(input.contact)
        .bind(input.adharcard_no)
        .bind(input.photo)
        .fetch_one(&data.pool)
        .await?;

    transaction.commit().await?;
    serde_json::to_value(String::from("Successful")).map_err(ErrorData::from)
}

pub async fn delete_staff(id: String, data: AppState) -> Result<Value, ErrorData> {
    let transaction = data.pool.begin().await?;
    let query = "DELETE FROM staff WHERE id = $1 RETURNING id";
    let _insert_query: i32 = sqlx::query_scalar(query)
        .bind(id)
        .fetch_one(&data.pool)
        .await?;
    transaction.commit().await?;
    serde_json::to_value(String::from("Successful")).map_err(ErrorData::from)
}
