use serde::{Deserialize, Serialize};
use sqlx;

#[derive(Deserialize, Serialize, Debug, Clone, Default, sqlx::FromRow)]
pub struct Home {
    pub id: String,
    pub wing: String,
    pub no: i32,
}
