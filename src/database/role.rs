use serde::{Deserialize, Serialize};
use sqlx;
use ts_rs::TS;

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow, TS, PartialEq, Eq)]
pub struct Roles {
    pub user_id: String,
    pub role: UserRole,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserRoleWrapper(pub UserRole);
use std::fmt;

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::Type, TS, PartialEq, Eq)]
#[sqlx(type_name = "role", rename_all = "lowercase")]
#[ts(export, export_to = "bindings/common/UserRole.ts")]
pub enum UserRole {
    Ranted,
    Admin,
    Guest,
    Maid,
    ServiceProvider,
    Security,
    Member,
    SuperAdmin,
    User,
}

impl fmt::Display for UserRole {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UserRole::Admin => write!(f, "admin"),
            UserRole::Guest => write!(f, "guest"),
            UserRole::Maid => write!(f, "maid"),
            UserRole::ServiceProvider => write!(f, "serviceprovider"),
            UserRole::Security => write!(f, "security"),
            UserRole::Member => write!(f, "member"),
            UserRole::SuperAdmin => write!(f, "superadmin"),
            UserRole::Ranted => write!(f, "ranted"),
            UserRole::User => write!(f, "user"),
        }
    }
}

#[derive(sqlx::FromRow)]
pub struct RolesRow {
    pub role: UserRole,
}

#[derive(sqlx::FromRow)]
pub struct Role {
    pub id: String,
    pub user_id: String,
    pub role: UserRole,
}
