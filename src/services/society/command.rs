use crate::{
    app_state::AppState,
    dto::society::{Id, SocietyInput},
    method::convention::ErrorData,
    utils::biscuit::check_role::check_role_exist,
};
use actix_web::web;
use anyhow::Result;
use biscuit_auth::Biscuit;
use chrono::Utc;
use rand::Rng;
use serde_json::Value;
use sqids::Sqids;
use uuid::Uuid;

pub async fn create_society(
    input: SocietyInput,
    data: AppState,
    biscuit: web::ReqData<Biscuit>,
) -> Result<Value, ErrorData> {
    if check_role_exist(vec!["superadmin".to_string()], &biscuit).is_ok() {
        let sqids = Sqids::default();
        let mut rng = rand::thread_rng();

        let n1: u8 = rng.gen();
        let id = sqids.encode(&[n1.into(), 2, 3]).unwrap();
        println!("{}", id);
        let mut tr = data.pool.begin().await?;
        let address_insert_query =
            "INSERT INTO address(id,pin_code,city,state,country,address_line1,address_line2)
            VALUES($1,$2,$3,$4,$5,$6,$7) RETURNING id";

        let address_id: String = sqlx::query_scalar(address_insert_query)
            .bind(Uuid::new_v4().as_simple().to_string())
            .bind(&input.address.pin_code)
            .bind(&input.address.city)
            .bind(&input.address.state)
            .bind(&input.address.country)
            .bind(&input.address.address_line1)
            .bind(&Some(input.address.address_line2))
            .fetch_one(tr.as_mut())
            .await?;

        let insert_query = "INSERT INTO society(id,name,address_id,created_by,created_at,allowed_attempts,maintenance_per_month) VALUES($1,$2,$3,$4,$5,$6,$7)";
        let _ = sqlx::query(insert_query)
            .bind(&id)
            .bind(&input.name)
            .bind(address_id)
            .bind(Uuid::new_v4().as_simple().to_string())
            .bind(Utc::now().to_string())
            .bind(&input.allowed_attempts)
            .bind(&input.maintenance_per_month)
            .execute(tr.as_mut())
            .await?;
        tr.commit().await?;

        serde_json::to_value(Id {
            id: id,
            message: String::from("Successful"),
        })
        .map_err(ErrorData::from)
    } else {
        Err(ErrorData {
            message: String::from("Authorization Error"),
            data: Value::Null,
            code: -32600,
        })
    }
}

pub async fn update_society(
    input: SocietyInput,
    id: String,
    data: AppState,
    biscuit: web::ReqData<Biscuit>,
) -> Result<Value, ErrorData> {
    if check_role_exist(
        vec!["admin".to_string(), "superadmin".to_string()],
        &biscuit,
    )
    .is_ok()
    {
        let mut transaction = data.pool.begin().await?;

        let update_society_query = "UPDATE society SET name = $1, created_by = $2, created_at = $3, allowed_attempts = $4, maintenance_per_month = $5 WHERE id = $6 RETURNING address_id";

        let address_id: String = sqlx::query_scalar(update_society_query)
            .bind(input.name)
            .bind(Uuid::new_v4().as_simple().to_string())
            .bind(Utc::now().to_string())
            .bind(input.allowed_attempts)
            .bind(input.maintenance_per_month)
            .bind(id)
            .fetch_one(transaction.as_mut())
            .await?;

        let address_query = "UPDATE address SET  pin_code = $1, city = $2 , state = $3 , country = $4 , address_line1 = $5 , address_line2 = $6 WHERE id = $7";

        let _ = sqlx::query(address_query)
            .bind(input.address.pin_code)
            .bind(input.address.city)
            .bind(input.address.state)
            .bind(input.address.country)
            .bind(input.address.address_line1)
            .bind(input.address.address_line2)
            .bind(address_id)
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

pub async fn delete_society(
    id: String,
    data: AppState,
    biscuit: web::ReqData<Biscuit>,
) -> Result<Value, ErrorData> {
    if check_role_exist(
        vec!["admin".to_string(), "superadmin".to_string()],
        &biscuit,
    )
    .is_ok()
    {
        let mut transaction = data.pool.begin().await?;

        // Get the address ID associated with the society
        let get_address_id_query = "
            SELECT address_id
            FROM society
            WHERE id = $1;
        ";

        let address_id: Option<String> = sqlx::query_scalar(get_address_id_query)
            .bind(id.clone())
            .fetch_optional(transaction.as_mut())
            .await?;

        if let Some(address_id) = address_id {
            // Delete the society row first to remove the reference
            let delete_society_query = "
                DELETE FROM society
                WHERE id = $1;
            ";

            sqlx::query(delete_society_query)
                .bind(id.clone())
                .execute(transaction.as_mut())
                .await?;

            // Now delete the associated address row
            let delete_address_query = "
                DELETE FROM address
                WHERE id = $1;
            ";

            sqlx::query(delete_address_query)
                .bind(address_id)
                .execute(transaction.as_mut())
                .await?;
        }

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
