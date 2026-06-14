use anyhow::{Ok, Result};
use chrono::Utc;
use dotenv::var;
use log::info;
use sqlx::PgPool;

pub async fn first_society_setup(pool: PgPool) -> Result<()> {
    info!(target: "Setup", "Creating First User");
    let first_user_auth_id = var("FIRST_USER_AUTH_ID").expect("first user auth id is not in env");
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) from users")
        .fetch_one(&pool)
        .await?;
    if row.0 == 0 {
        let society_id = var("SOCIETY_ID").expect("SOCIETY_ID is not in env");
        let mut tr = pool.begin().await?;
        let address_insert_query =
            "INSERT INTO address(id,pin_code,city,state,country,address_line1,address_line2)
        VALUES($1,$2,$3,$4,$5,$6,$7) RETURNING id";

        let address_id: String = sqlx::query_scalar(address_insert_query)
        .bind(uuid::Uuid::new_v4().as_simple().to_string())
        .bind(String::from("380015"))
        .bind(String::from("Ahmadabad"))
        .bind(String::from("Gujarat"))
        .bind(String::from("India"))
        .bind(String::from("Sachin Tower, E-708, Titamium City Center, Nr, 100 Feet Anand Nagar Rd, Ahmedabad, Gujarat"))
        .bind(&Some(String::from("Titamium City Center")))
        .fetch_one(tr.as_mut())
        .await?;

        let insert_query = "INSERT INTO society(id,name,address_id,created_by,created_at,allowed_attempts,maintenance_per_month) VALUES($1,$2,$3,$4,$5,$6,$7)";
        let _ = sqlx::query(insert_query)
            .bind(society_id)
            .bind(String::from("IGate"))
            .bind(address_id)
            .bind(first_user_auth_id)
            .bind(Utc::now().to_string())
            .bind(5)
            .bind(100000)
            .execute(tr.as_mut())
            .await?;
        tr.commit().await?;
    } else {
        info!(target: "Setup" , "Society data already exist. Please delete file to reset");
    }
    Ok(())
}
