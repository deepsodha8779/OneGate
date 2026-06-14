use serde::{Deserialize, Serialize};
use sqlx;

#[derive(Deserialize, Serialize, Debug, Clone, Default, sqlx::FromRow)]
pub struct Guest {
    pub id: String,
    pub user_id: String,
    pub society_id:String,
    pub flat_name: String,
}

