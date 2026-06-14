use crate::database::auth::LoginUser;
use crate::database::role::RolesRow;
use crate::method::convention::ErrorData;
use crate::utils::password_helper::{hash_password, verify_password};
use crate::utils::sms_helper::send_sms;
use crate::{
    app_state::AppState,
    dto::auth_types::{AuthenticatedUser, Imageurl, LoginMobile, SignUpDTO},
    utils::biscuit::helper::create_token,
};
use anyhow::Result;
use biscuit_auth::KeyPair;
use chrono::Local;
use log::info;
use rand::seq::IteratorRandom;
use serde_json::Value;
use uuid::Uuid;

pub async fn login_submit(input: LoginMobile, state: AppState) -> Result<Value, ErrorData> {
    let d_user: Option<LoginUser> = sqlx::query_as::<_, LoginUser>(
        "SELECT id, mobile_number, password_hash FROM userauth WHERE mobile_number = $1",
    )
    .bind(&input.mobile_number)
    .fetch_optional(&state.pool)
    .await?;

    if let Some(user) = d_user {
        let stored_hash = &user.password_hash;
        let user_id = user.id;
        let is_valid = verify_password(&stored_hash, &input.password).unwrap_or(false);

        if is_valid {
            let query = "SELECT role FROM auth_roles WHERE user_auth_id = $1";
            let rows = sqlx::query_as::<_, RolesRow>(query)
                .bind(&user_id)
                .fetch_all(&state.pool)
                .await?;
            let roles: Vec<String> = rows.into_iter().map(|row| row.role.to_string()).collect();
            let vec_of_refs: Vec<&str> = roles.iter().map(|s| s.as_str()).collect();

            let token = create_token(
                &KeyPair::from(&state.private_key),
                vec_of_refs,
                &user_id,
                &input.society_id,
            )?;
            serde_json::to_value(AuthenticatedUser {
                id: user_id.to_string(),
                token: token,
                selected_language: "EN".to_string(),
                role: roles,
            })
            .map_err(ErrorData::from)
        } else {
            Err(ErrorData {
                message: String::from("Password doesn't match"),
                data: Value::Null,
                code: -32600,
            })
        }
    } else {
        Err(ErrorData {
            message: String::from("User Not Found"),
            data: Value::Null,
            code: -32600,
        })
    }
}

pub async fn signup_submit(input: SignUpDTO, state: AppState) -> Result<Value, ErrorData> {
    let otp = (100000..999999)
        .choose(&mut rand::thread_rng())
        .unwrap()
        .to_string();
    info!("OTP:-{:#?}", otp);
    let message = format!("Your OTP is {}", otp);
    let hash = hash_password(&input.password).unwrap();
    let is_valid = verify_password(&hash, &input.confirm_password).unwrap_or(false);

    send_sms(&message, vec![input.mobile_number.clone()], "iGate").await?;
    if is_valid == true {
        let mut tr = state.pool.begin().await?;
        let u_id = Uuid::new_v4().as_simple().to_string();
        let insert_query =
            "INSERT INTO userauth(id,username,password_hash,mobile_number,created_at)
         VALUES($1,$2,$3,$4,$5) RETURNING id";

        let user_auth_id: String = sqlx::query_scalar(insert_query)
            .bind(u_id.clone())
            .bind(input.user_name)
            .bind(hash)
            .bind(input.mobile_number.clone())
            .bind(Local::now().to_string())
            .fetch_one(tr.as_mut())
            .await?;

        let roles_query = "INSERT INTO auth_roles(id,user_auth_id,role)
            VALUES($1,$2,$3)";
        let _ = sqlx::query(roles_query)
            .bind(uuid::Uuid::new_v4().as_simple().to_string())
            .bind(user_auth_id)
            .bind(input.role)
            .execute(tr.as_mut())
            .await?;

        tr.commit().await?;
        serde_json::to_value(String::from("SignUp Successful")).map_err(ErrorData::from)
    } else {
        Err(ErrorData {
            message: String::from("password and confirm password is not match"),
            data: Value::Null,
            code: -32600,
        })
    }
}

pub async fn url_submit(_data: AppState) -> Result<Value, ErrorData> {
    serde_json::to_value(vec![
        Imageurl{
            photo_url:"https://www.google.com/imgres?imgurl=https%3A%2F%2Fimg.freepik.com%2Fpremium-vector%2Fhappy-men-women_24911-64441.jpg&tbnid=mxFxAVxL8QdGXM&vet=10CAYQxiAoBmoXChMIoOTIipGJgAMVAAAAAB0AAAAAEAg..i&imgrefurl=https%3A%2F%2Fwww.freepik.com%2Ffree-photos-vectors%2Fdiverse-society%2F33&docid=R-vizsU2lcb01M&w=626&h=468&itg=1&q=society%20images&ved=0CAYQxiAoBmoXChMIoOTIipGJgAMVAAAAAB0AAAAAEAg".to_string(),  
        }
            ])
            .map_err(ErrorData::from)
}
