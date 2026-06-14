use crate::dto::common::address::Address;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::common::address::AddressInput;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS, sqlx::FromRow)]
#[ts(export, export_to = "bindings/Society.ts")]
pub struct Society {
    pub id: String,
    pub name: String,
    pub address: Address,
    pub allowed_attempts: i32,
    pub maintenance_per_month: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS, sqlx::FromRow)]
#[ts(export, export_to = "bindings/SocietyInput.ts")]
pub struct SocietyInput {
    pub name: String,
    pub allowed_attempts: i32,
    pub maintenance_per_month: i32,
    pub address: AddressInput,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS, sqlx::FromRow)]
#[ts(export, export_to = "bindings/society/UpdateSociety.ts")]
pub struct UpdateSociety {
    pub id: String,
    pub input: SocietyInput,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS, sqlx::FromRow)]
#[ts(export, export_to = "bindings/society/Id.ts")]
pub struct Id {
    pub id: String,
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS, sqlx::FromRow)]
#[ts(export, export_to = "bindings/society/IdInput.ts")]
pub struct IdInput {
    pub id: String,
}
