use serde::{Deserialize, Serialize};
use sqlx;

#[derive(Deserialize, Serialize, Debug, Clone, Default, sqlx::FromRow)]
pub struct MaidDetails {
    pub id: String,
    pub user_id: String,
}
