// use validator::Validate;
use super::common::user::{User, UserInput};
use chrono::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS, sqlx::FromRow)]
#[ts(export, export_to = "bindings/Security.ts")]
pub struct Security {
    pub id: String,
    pub user: User,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS, sqlx::FromRow)]
#[ts(export, export_to = "bindings/security/SecurityInput.ts")]
pub struct SecurityInput {
    pub user: UserInput,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, PartialEq, Eq, TS)]
#[ts(export, export_to = "bindings/security/UpdateSecurity.ts")]
pub struct UpdateSecurity {
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub id: String,
    pub input: SecurityInput,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, PartialEq, Eq, TS)]
#[ts(export, export_to = "bindings/security/Id.ts")]
pub struct Id {
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub id: String,
}
