use crate::{database::role::UserRole, utils::password_helper::hash_password};
use anyhow::{Ok, Result};
use chrono::Local;
use dotenv::var;
use log::info;
use sqlx::PgPool;

pub async fn first_user_auth_setup(pool: PgPool) -> Result<()> {
    info!(target: "Setup", "Creating First User Auth");
    let first_user_mobile = var("FIRST_USER_MOBILE").expect("mobile number is not in env");
    let first_user_first_name = var("FIRST_USER_FIRST_NAME").expect("first_name is not in env");
    
    
    let first_user_auth_id = var("FIRST_USER_AUTH_ID").expect("Photo URL is not in env");
    let first_user_password = var("FIRST_USER_PASSWORD").expect("Photo URL is not in env");
    let hash = hash_password(&first_user_password).unwrap();
    info!(target: "Setup", "Inserting First UserAuth data");
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) from userauth")
        .fetch_one(&pool)
        .await?;

    if row.0 == 0 {
        let mut tr = pool.begin().await?;
        let insert_query =
            "INSERT INTO userauth(id,username,password_hash,mobile_number,created_at)
         VALUES($1,$2,$3,$4,$5) RETURNING id";

        let user_auth_id: String = sqlx::query_scalar(insert_query)
            .bind(first_user_auth_id)
            .bind(first_user_first_name)
            .bind(hash)
            .bind(first_user_mobile)
            .bind(Local::now().to_string())
            .fetch_one(&pool)
            .await?;

        let roles_query = "INSERT INTO auth_roles(id,user_auth_id,role)
            VALUES($1,$2,$3)";
        let _ = sqlx::query(roles_query)
            .bind(uuid::Uuid::new_v4().as_simple().to_string())
            .bind(user_auth_id)
            .bind(UserRole::SuperAdmin)
            .execute(tr.as_mut())
            .await?;

        tr.commit().await?;
    } else {
        info!(target: "Setup" , "users data already exist. Please delete file to reset");
    }
    Ok(())
}
