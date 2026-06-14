use serde::{Deserialize, Serialize};
use sqlx;

#[derive(Deserialize, Serialize, Debug, Clone, Default, sqlx::FromRow)]
pub struct Amenity {
    pub id: String,
    pub name: String,
    pub description: String,
    pub start_time: String,
    pub end_time: String,
}
