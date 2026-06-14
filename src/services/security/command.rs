use crate::{
    app_state::AppState, dto::security::SecurityInput, method::convention::ErrorData,
    utils::biscuit::check_role::check_role_exist,
};
use actix_web::web;
use anyhow::Result;
use biscuit_auth::Biscuit;
use serde_json::Value;
use uuid::Uuid;

pub async fn add_security(
    input: SecurityInput,
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

        let user_detail_query ="INSERT INTO user_details(id,first_name,last_name,aadhar_number,photo_url,contact_number,contact_type,email)
                                     VALUES($1,$2,$3,$4,$5,$6,$7,$8) RETURNING id";
        let user_detail_id: String = sqlx::query_scalar(user_detail_query)
            .bind(Uuid::new_v4().as_simple().to_string())
            .bind(input.user.user_detail.first_name)
            .bind(input.user.user_detail.last_name)
            .bind(input.user.user_detail.aadhar_number)
            .bind(input.user.user_detail.photo_url)
            .bind(input.user.user_detail.contact_number)
            .bind(input.user.user_detail.contact_type)
            .bind(input.user.user_detail.email)
            .fetch_one(tr.as_mut())
            .await?;

        let insert_query = "INSERT INTO users(id,user_detail_id,society_id,is_block,is_deleted)
        VALUES($1,$2,$3,$4,$5) RETURNING id";
        let user_id: String = sqlx::query_scalar(insert_query)
            .bind(Uuid::new_v4().as_simple().to_string())
            .bind(user_detail_id)
            .bind(society_id)
            .bind(input.user.is_block)
            .bind(input.user.is_deleted)
            .fetch_one(tr.as_mut())
            .await?;

        let security_query = "INSERT INTO security_details (id,user_id,start_time,end_time)
        VALUES($1,$2,$3,$4)";

        let _ = sqlx::query(security_query)
            .bind(uuid::Uuid::new_v4().as_simple().to_string())
            .bind(user_id)
            .bind(input.start_time)
            .bind(input.end_time)
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

pub async fn update_security(
    input: SecurityInput,
    id: String,
    data: AppState,
    _society_id: String,
    biscuit: web::ReqData<Biscuit>,
) -> Result<Value, ErrorData> {
    if check_role_exist(
        vec!["admin".to_string(), "superadmin".to_string()],
        &biscuit,
    )
    .is_ok()
    {
        let mut transaction = data.pool.begin().await?;

        // Update security_details table
        let security_details_query = "UPDATE security_details
                                      SET start_time = $1 , end_time = $2
                                      WHERE id = $3";
        sqlx::query(security_details_query)
            .bind(input.start_time)
            .bind(input.end_time)
            .bind(id.clone())
            .execute(transaction.as_mut())
            .await?;

        // Update users table
        let user_query = "UPDATE users
                          SET is_block = $1, is_deleted = $2
                          WHERE id = (SELECT user_id FROM security_details WHERE id = $3)";
        sqlx::query(user_query)
            .bind(input.user.is_block)
            .bind(input.user.is_deleted)
            .bind(id.clone())
            .execute(transaction.as_mut())
            .await?;

        let user_id_query = "SELECT user_id FROM security_details WHERE id = $1";
        let user_id: String = sqlx::query_scalar(user_id_query)
            .bind(id.clone())
            .fetch_one(transaction.as_mut())
            .await?;

        // Update user_details table
        let user_details_query = "UPDATE user_details
                                  SET first_name = $1, last_name = $2, aadhar_number = $3, photo_url = $4,
                                      contact_number = $5, contact_type = $6, email = $7
                                  WHERE id = (SELECT user_detail_id FROM users WHERE id = $8)";
        sqlx::query(user_details_query)
            .bind(input.user.user_detail.first_name)
            .bind(input.user.user_detail.last_name)
            .bind(input.user.user_detail.aadhar_number)
            .bind(input.user.user_detail.photo_url)
            .bind(input.user.user_detail.contact_number)
            .bind(input.user.user_detail.contact_type)
            .bind(input.user.user_detail.email)
            .bind(user_id)
            .execute(transaction.as_mut())
            .await?;

        // Commit the transaction
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

pub async fn delete_security(
    id: String,
    data: AppState,
    _society_id: String,
    biscuit: web::ReqData<Biscuit>,
) -> Result<Value, ErrorData> {
    if check_role_exist(
        vec!["admin".to_string(), "superadmin".to_string()],
        &biscuit,
    )
    .is_ok()
    {
        let mut tx = data.pool.begin().await?;

        let delete_security_details_query = "
    DELETE FROM security_details
    WHERE id = $1
    RETURNING id, user_id;
";

        let deleted_security_details: Vec<(String, String)> =
            sqlx::query_as(delete_security_details_query)
                .bind(id.clone())
                .fetch_all(tx.as_mut())
                .await?;

        let deleted_user_ids: Vec<String> = deleted_security_details
            .iter()
            .map(|(_, user_id)| user_id.clone())
            .collect();

        let delete_users_query = "
    DELETE FROM users
    WHERE id = ANY($1::VARCHAR[])
    RETURNING id;
";

        sqlx::query(delete_users_query)
            .bind(&deleted_user_ids)
            .execute(tx.as_mut())
            .await?;

        let delete_user_details_query = "
    DELETE FROM user_details
    WHERE id = ANY($1::VARCHAR[]);
";

        sqlx::query(delete_user_details_query)
            .bind(&deleted_user_ids)
            .execute(tx.as_mut())
            .await?;
        tx.commit().await?;

        serde_json::to_value(String::from("Successful")).map_err(ErrorData::from)
    } else {
        Err(ErrorData {
            message: String::from("Authorization Error"),
            data: Value::Null,
            code: -32600,
        })
    }
}
