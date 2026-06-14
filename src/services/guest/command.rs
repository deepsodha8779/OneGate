use anyhow::Result;
use chrono::Utc;

use crate::{
    app_state::AppState, database::contact::ContactTypes, dto::guest::GuestInput,
    method::convention::ErrorData,
};

use serde_json::Value;
use uuid::Uuid;

pub async fn add_guest(input: GuestInput, data: AppState) -> Result<Value, ErrorData> {
    let mut tr = data.pool.begin().await?;
    let address_insert_query =
        "INSERT INTO address(id,pin_code,city,state,country,address_line1,address_line2)
            VALUES($1,$2,$3,$4,$5,$6,$7) RETURNING id";
    let address_id: String = sqlx::query_scalar(address_insert_query)
        .bind(uuid::Uuid::new_v4().as_simple().to_string())
        .bind(&input.society.address.pin_code)
        .bind(&input.society.address.city)
        .bind(&input.society.address.state)
        .bind(&input.society.address.country)
        .bind(&input.society.address.address_line1)
        .bind(&Some(input.society.address.address_line2))
        .fetch_one(tr.as_mut())
        .await?;

    println!("address_id={:?}", address_id);

    let society_insert_query = "INSERT INTO society(id,name,address_id,created_by,created_at,allowed_attempts,maintenance_per_month)
                                              VALUES($1,$2,$3,$4,$5,$6,$7) RETURNING id";
    let society_id: String = sqlx::query_scalar(society_insert_query)
        .bind(Uuid::new_v4().as_simple().to_string())
        .bind(&input.society.name)
        .bind(address_id)
        .bind(Uuid::new_v4().as_simple().to_string())
        .bind(Utc::now().to_string())
        .bind(&input.society.allowed_attempts)
        .bind(&input.society.maintenance_per_month)
        .fetch_one(tr.as_mut())
        .await?;

    let user_detail_query ="INSERT INTO user_details(id,first_name,last_name,aadhar_number,photo_url,contact_number,contact_type,email)
                                     VALUES($1,$2,$3,$4,$5,$6,$7,$8) RETURNING id";
    let user_detail_id: String = sqlx::query_scalar(user_detail_query)
        .bind(Uuid::new_v4().as_simple().to_string())
        .bind(input.user.user_detail.first_name)
        .bind(input.user.user_detail.last_name)
        .bind(input.user.user_detail.aadhar_number)
        .bind(input.user.user_detail.photo_url)
        .bind(input.user.user_detail.contact_number)
        .bind(ContactTypes::Mobile)
        .bind(input.user.user_detail.email)
        .fetch_one(tr.as_mut())
        .await?;

    let insert_query = "INSERT INTO users(id,user_detail_id,is_block,is_deleted)
                VALUES($1,$2,$3,$4) RETURNING id";
    let user_id: String = sqlx::query_scalar(insert_query)
        .bind(Uuid::new_v4().as_simple().to_string())
        .bind(user_detail_id)
        .bind(false)
        .bind(false)
        .fetch_one(tr.as_mut())
        .await?;

    let guest_query = "INSERT INTO guest(id,user_id,society_id,flat_name) VALUES($1,$2,$3,$4)";
    let _ = sqlx::query(guest_query)
        .bind(uuid::Uuid::new_v4().as_simple().to_string())
        .bind(user_id)
        .bind(society_id)
        .bind(input.flat_name)
        .execute(tr.as_mut())
        .await?;
    tr.commit().await?;
    serde_json::to_value(String::from("Successful")).map_err(ErrorData::from)
}

pub async fn update_guest(
    _input: GuestInput,
    _id: String,
    data: AppState,
) -> Result<Value, ErrorData> {
    let transaction = data.pool.begin().await?;
    let query = "UPDATE guest SET
    flat_name = $1,
    total_guest =$2,
    WHERE id = $3 RETURNING id";
    let _update_query: i32 = sqlx::query_scalar(query).fetch_one(&data.pool).await?;
    transaction.commit().await?;
    serde_json::to_value(String::from("Successful")).map_err(ErrorData::from)
}

pub async fn delete_guest(_id: String, _data: AppState) -> Result<Value, ErrorData> {
    serde_json::to_value(String::from("Successful")).map_err(ErrorData::from)
}
