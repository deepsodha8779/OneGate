use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
// use validator::Validate;
use ts_rs::TS;

#[derive(Deserialize, Serialize, Clone, Debug, TS)]
#[ts(export, export_to = "bindings/Maintenance.ts")]
pub struct Maintenance {
    pub amount: f32,
    pub duration_months: i8,
    pub starting_month: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, TS)]
#[ts(export, export_to = "bindings/FlatMaintenance.ts")]
pub struct FlatMaintenance {
    pub maintenance: Maintenance,
    pub paid_at: DateTime<Local>,
}
