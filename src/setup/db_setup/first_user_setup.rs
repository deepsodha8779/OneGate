use crate::database::{contact::ContactTypes, role::UserRole};
use anyhow::{Ok, Result};
use dotenv::var;
use log::info;
use sqlx::PgPool;

pub async fn first_user_setup(pool: PgPool) -> Result<()> {
    info!(target: "Setup", "Creating First User");
    let first_user_mobile = var("FIRST_USER_MOBILE").expect("mobile number is not in env");
    let first_user_first_name = var("FIRST_USER_FIRST_NAME").expect("first_name is not in env");

    let first_user_last_name = var("FIRST_USER_LAST_NAME").expect("last_name is not in env");

    let first_user_email = var("FIRST_USER_EMAIL").expect("email is not in env");
    let first_user_aardhar_number =
        var("FIRST_USER_AADHAR_NUMBER").expect("AADHAR NUMBER is not in env");

    let first_user_id = var("FIRST_USER_ID").expect("ID is not in env");
    let first_user_photo_url = var("FIRST_USER_PHOTO_URL").expect("Photo URL is not in env");

    info!(target: "Setup", "Inserting First User data");
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) from users")
        .fetch_one(&pool)
        .await?;

    if row.0 == 0 {
        let mut tr = pool.begin().await?;

        let user_insert_query = "INSERT INTO user_details(id,first_name,last_name,aadhar_number,photo_url,contact_number,contact_type,email)
        VALUES($1,$2,$3,$4,$5,$6,$7,$8) RETURNING id";
        let user_detail_id: String = sqlx::query_scalar(user_insert_query)
            .bind("1A".to_string())
            .bind(first_user_first_name)
            .bind(first_user_last_name)
            .bind(first_user_aardhar_number)
            .bind(first_user_photo_url)
            .bind(first_user_mobile)
            .bind(ContactTypes::Mobile)
            .bind(first_user_email)
            .fetch_one(&pool)
            .await?;

        let insert_query = "INSERT INTO users(id,user_detail_id,is_block,is_deleted)
            VALUES($1,$2,$3,$4) RETURNING id ";
        let user_id: String = sqlx::query_scalar(insert_query)
            .bind(first_user_id)
            .bind(user_detail_id)
            .bind(false)
            .bind(false)
            .fetch_one(&pool)
            .await?;
        let roles_query = "INSERT INTO roles(id,user_id,role)
            VALUES($1,$2,$3)";
        let _ = sqlx::query(roles_query)
            .bind(uuid::Uuid::new_v4().as_simple().to_string())
            .bind(user_id)
            .bind(UserRole::SuperAdmin)
            .execute(tr.as_mut())
            .await?;
        tr.commit().await?;
    } else {
        info!(target: "Setup" , "users data already exist. Please delete file to reset");
    }
    Ok(())
}
