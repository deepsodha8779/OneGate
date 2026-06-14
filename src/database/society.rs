use serde::{Deserialize, Serialize};
use sqlx;
use ts_rs::TS;

#[derive(Deserialize, Serialize, Debug, Clone, Default, sqlx::FromRow)]
pub struct Society {
    pub id: String,
    pub name: String,
    pub address_id: String,
    pub created_by: String, //user_id
    pub created_at: String,
    pub allowed_attempts: u16,
    pub maintenance_per_month: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS, sqlx::FromRow)]
#[ts(export, export_to = "bindings/society/SocietyData.ts")]
pub struct SocietyData {
    pub id: String,
    pub name: String,
    pub allowed_attempts: i32,
    pub maintenance_per_month: i32,
    pub address_id: String,
    pub pin_code: String,
    pub city: String,
    pub state: String,
    pub address_line1: String,
    pub country: String,
    pub address_line2: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default, sqlx::FromRow)]
pub struct SocietyAdmin {
    society_id: String,
    user_id: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, Default, sqlx::FromRow)]
pub struct SocietyMember {
    society_id: String,
    user_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default, sqlx::FromRow)]
pub struct SocietyMaid {
    society_id: String,
    maid_id: String,
}
