use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx;

#[derive(Deserialize, Serialize, Debug, Clone, Default, sqlx::FromRow)]
pub struct UserAuth {
    pub id: String,
    pub mobile_number: String,
    pub otp: String,
    pub otp_sent: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default, sqlx::FromRow)]
pub struct Auth {
    id: String,
    user_id: String,
    otp: String, // Generated OTP
    otp_sent: DateTime<Local>,
    attempts: u16,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default, sqlx::FromRow)]
pub struct LoginUser {
    pub id: String,
    pub mobile_number: String,
    pub password_hash: String,
}
