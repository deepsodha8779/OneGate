use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::database::contact::ContactTypes;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS, sqlx::FromRow, sqlx::Type)]
#[ts(export, export_to = "bindings/user_details/UserDetail.ts")]
pub struct UserDetail {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub aadhar_number: String,
    pub photo_url: String,
    pub contact_number: String,
    pub contact_type: ContactTypes,
    pub email: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS, sqlx::FromRow)]
#[ts(export, export_to = "bindings/user_details/UserDetailInput.ts")]
pub struct UserDetailInput {
    pub first_name: String,
    pub last_name: String,
    pub aadhar_number: String,
    pub photo_url: String,
    pub contact_number: String,
    pub contact_type: ContactTypes,
    pub email: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS, sqlx::FromRow)]
#[ts(export, export_to = "bindings/user_details/UpdateUserDetail.ts")]
pub struct UpdateUserDetail {
    pub id: String,
    pub input: UserDetailInput,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS, sqlx::FromRow)]
#[ts(export, export_to = "bindings/user_details/Id.ts")]
pub struct Id {
    pub id: String,
}
