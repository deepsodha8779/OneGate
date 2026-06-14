use serde::{Deserialize, Serialize};
use sqlx;

#[derive(Deserialize, Serialize, Debug, Clone, Default, sqlx::FromRow)]
pub struct SocietyGuest {
    pub society_id: String,
    pub guest_id: String,
}
