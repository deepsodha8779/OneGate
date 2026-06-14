use serde::{Deserialize, Serialize};
use sqlx;

#[derive(Deserialize, Serialize, Debug, Clone, Default, sqlx::FromRow)]
pub struct Address {
    pub id: String,
    pub pin_code: String,
    pub city: String,
    pub state: String,
    pub address_line1: String,
    pub country: String,
    pub address_line2: Option<String>,
}
