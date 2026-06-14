use chrono::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS)]
#[ts(export, export_to = "bindings/amenity/Amenity.ts ")]
pub struct Amenity {
    pub id: String,
    pub name: String,
    pub description: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS, sqlx::FromRow)]
#[ts(export, export_to = "bindings/AmenityInput.ts")]
pub struct AmenityInput {
    pub name: String,
    pub description: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS, sqlx::FromRow)]
#[ts(export, export_to = "bindings/amenity/UpdateAmenity.ts")]
pub struct UpdateAmenity {
    pub id: String,
    pub input: AmenityInput,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS, sqlx::FromRow)]
#[ts(export, export_to = "bindings/amenity/Id.ts")]
pub struct Id {
    pub id: String,
}
