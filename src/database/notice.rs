use serde::{Deserialize, Serialize};
use sqlx;

#[derive(Deserialize, Serialize, Debug, Clone, Default, sqlx::FromRow)]
pub struct Notice {
    pub id: String,
    pub title: String,
    pub notice_description: String,
}
