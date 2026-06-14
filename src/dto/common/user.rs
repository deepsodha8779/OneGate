use crate::{
    database::role::UserRole,
    dto::user_details::{UserDetail, UserDetailInput},
};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS, sqlx::FromRow)]
#[ts(export, export_to = "bindings/common/user/User.ts")]
pub struct User {
    pub id: String,
    pub user_details: UserDetail,
    pub is_block: bool,
    pub is_deleted: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS, sqlx::FromRow)]
#[ts(export, export_to = "bindings/common/user/UserInput.ts")]
pub struct UserInput {
    pub user_detail: UserDetailInput,
    pub is_block: bool,
    pub is_deleted: bool,
    pub role: UserRole,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS, sqlx::FromRow)]
#[ts(export, export_to = "bindings/common/user/UpdateUser.ts")]
pub struct UpdateUser {
    pub id: String,
    pub update_user: UserInput,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS, sqlx::FromRow)]
#[ts(export, export_to = "bindings/common/user/Id.ts")]
pub struct Id {
    pub id: String,
}
