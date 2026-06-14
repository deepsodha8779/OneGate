use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

use crate::database::role::UserRole;

use super::common::user::User;

#[derive(Deserialize, Serialize, Debug, Clone, Validate, PartialEq, Eq, TS)]
#[ts(export, export_to = "bindings/SignUpDTO.ts")]
pub struct SignUpDTO {
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub user_name: String,
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub mobile_number: String,
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub password: String,
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub confirm_password: String,
    pub role: UserRole,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, PartialEq, Eq, TS)]
#[ts(export, export_to = "bindings/LoginMobile.ts")]
pub struct LoginMobile {
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub mobile_number: String,
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub password: String,
    pub society_id: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, TS)]
#[ts(export, export_to = "bindings/AuthenticatedUser.ts")]
pub struct AuthenticatedUser {
    pub id: String,
    pub token: String,
    pub selected_language: String,
    pub role: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug, TS)]
#[ts(export, export_to = "bindings/Auth.ts")]
pub struct Auth {
    pub user: User,
    pub otp: String, // Generated OTP
    pub otp_sent: DateTime<Local>,
}

#[derive(Deserialize, Serialize, Clone, Debug, TS)]
#[ts(export, export_to = "bindings/OtpTable.ts")]
pub struct OtpTable {
    pub id: String,
    pub mobile_number: String,
    pub otp: String, // Generated OTP
    pub otp_sent: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, PartialEq, Eq, TS)]
#[ts(export, export_to = "bindings/Imageurl.ts")]
pub struct Imageurl {
    pub photo_url: String,
}
