use serde::{Deserialize, Serialize};
use sqlx;

#[derive(Deserialize, Serialize, Debug, Clone, Default, sqlx::FromRow)]
pub struct Complaint {
    pub id: String,
    pub complaint_by: String,
    pub complaint_title: String,
    pub complaint_description: String,
    pub status: String,
}
