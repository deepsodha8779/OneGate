use crate::{
    app_state::AppState, database::contact::ContactTypes, dto::member::MemberInput,
    method::convention::ErrorData,
};
use anyhow::Result;
use serde_json::Value;
use uuid::Uuid;

pub async fn add_member(input: MemberInput, data: AppState) -> Result<Value, ErrorData> {
    let mut tr = data.pool.begin().await?;

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

    let member_query = "INSERT INTO member_details (id,user_id)
    VALUES($1,$2)";

    let _ = sqlx::query(member_query)
        .bind(uuid::Uuid::new_v4().as_simple().to_string())
        .bind(user_id)
        .execute(tr.as_mut())
        .await?;
    tr.commit().await?;

    serde_json::to_value(String::from("Successful")).map_err(ErrorData::from)
}

pub async fn update_member(
    _input: MemberInput,
    _id: String,
    _data: AppState,
) -> Result<Value, ErrorData> {
    serde_json::to_value(String::from("Successful")).map_err(ErrorData::from)
}

pub async fn delete_member(_id: String, _data: AppState) -> Result<Value, ErrorData> {
    serde_json::to_value(String::from("Successful")).map_err(ErrorData::from)
}
