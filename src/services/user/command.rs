use actix_web::web;
use anyhow::Result;
use biscuit_auth::Biscuit;
use serde_json::Value;
use uuid::Uuid;

use crate::{
    app_state::AppState,
    database::contact::ContactTypes,
    dto::common::user::{UpdateUser, UserInput},
    method::convention::ErrorData,
    utils::biscuit::check_role::check_role_exist,
};

pub async fn add_user_submit(
    input: UserInput,
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

        let user_detail_query ="INSERT INTO user_details(id,first_name,last_name,aadhar_number,photo_url,contact_number,contact_type,email)
                                     VALUES($1,$2,$3,$4,$5,$6,$7,$8) RETURNING id";
        let user_detail_id: String = sqlx::query_scalar(user_detail_query)
            .bind(Uuid::new_v4().as_simple().to_string())
            .bind(input.user_detail.first_name)
            .bind(input.user_detail.last_name)
            .bind(input.user_detail.aadhar_number)
            .bind(input.user_detail.photo_url)
            .bind(input.user_detail.contact_number)
            .bind(ContactTypes::Mobile)
            .bind(input.user_detail.email)
            .fetch_one(tr.as_mut())
            .await?;

        let insert_query = "INSERT INTO users(id,user_detail_id,society_id,is_block,is_deleted)
                VALUES($1,$2,$3,$4,$5) RETURNING id";
        let user_id: String = sqlx::query_scalar(insert_query)
            .bind(Uuid::new_v4().as_simple().to_string())
            .bind(user_detail_id)
            .bind(society_id)
            .bind(input.is_block)
            .bind(input.is_deleted)
            .fetch_one(tr.as_mut())
            .await?;

        let roles_query = "INSERT INTO roles(id,user_id,role)
                VALUES($1,$2,$3)";

        let _ = sqlx::query(roles_query)
            .bind(uuid::Uuid::new_v4().as_simple().to_string())
            .bind(user_id)
            .bind(input.role)
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

pub async fn update_user(
    input: UpdateUser,
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
        let user_detail_id: String = sqlx::query_scalar(
            "SELECT users.user_detail_id FROM users WHERE id = $1 AND society_id = $2",
        )
        .bind(&input.id)
        .bind(&society_id)
        .fetch_one(&data.pool)
        .await?;

        let mut transaction = data.pool.begin().await?;

        let user_details_query = "UPDATE user_details SET first_name = $1, last_name = $2, aadhar_number = $3, photo_url = $4, contact_number = $5, contact_type = $6, email = $7 WHERE id = $8";

        let _ = sqlx::query(user_details_query)
            .bind(input.update_user.user_detail.first_name)
            .bind(input.update_user.user_detail.last_name)
            .bind(input.update_user.user_detail.aadhar_number)
            .bind(input.update_user.user_detail.photo_url)
            .bind(input.update_user.user_detail.contact_number)
            .bind(input.update_user.user_detail.contact_type)
            .bind(input.update_user.user_detail.email)
            .bind(user_detail_id)
            .execute(transaction.as_mut())
            .await?;

        let user_query = "UPDATE users SET is_block = $1, is_deleted = $2 WHERE id = $3";

        let _ = sqlx::query(user_query)
            .bind(input.update_user.is_block)
            .bind(input.update_user.is_deleted)
            .bind(input.id.clone())
            .execute(transaction.as_mut())
            .await?;

        let role_query = "UPDATE roles SET role = $1 WHERE user_id = $2";

        let _ = sqlx::query(role_query)
            .bind(input.update_user.role)
            .bind(input.id)
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

pub async fn delete_user(
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
        let mut tx = data.pool.begin().await?;

        // Get user detail ID
        let get_user_detail_id_query = "
            SELECT user_detail_id
            FROM users
            WHERE id = $1 AND society_id = $2;
        ";

        let user_detail_id: String = sqlx::query_scalar(get_user_detail_id_query)
            .bind(id.clone())
            .bind(society_id.clone())
            .fetch_one(tx.as_mut())
            .await?;

        // Delete associated roles first
        let delete_roles_query = "
            DELETE FROM roles
            WHERE user_id = $1;
        ";

        sqlx::query(delete_roles_query)
            .bind(id.clone())
            .execute(tx.as_mut())
            .await?;

        // Delete user from users table
        let delete_user_query = "
            DELETE FROM users
            WHERE id = $1 AND society_id = $2
            RETURNING id;
        ";

        let deleted_user_id: String = sqlx::query_scalar(delete_user_query)
            .bind(id.clone())
            .bind(society_id)
            .fetch_one(tx.as_mut())
            .await?;

        // Delete user details
        let delete_user_details_query = "
            DELETE FROM user_details
            WHERE id = $1;
        ";

        sqlx::query(delete_user_details_query)
            .bind(user_detail_id.clone())
            .execute(tx.as_mut())
            .await?;

        tx.commit().await?;

        serde_json::to_value(deleted_user_id).map_err(ErrorData::from)
    } else {
        Err(ErrorData {
            message: String::from("Authorization Error"),
            data: Value::Null,
            code: -32600,
        })
    }
}
