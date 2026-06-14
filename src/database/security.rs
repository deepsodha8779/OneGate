use serde::{Deserialize, Serialize};
use sqlx;
use ts_rs::TS;

use super::contact::ContactTypes;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS, sqlx::FromRow)]
#[ts(export, export_to = "bindings/service_provider/SecurityData.ts")]
pub struct SecurityData {
    pub id: String,
    pub start_time: String,
    pub end_time: String,
    pub user_id: String,
    pub is_block: bool,
    pub is_deleted: bool,
    pub user_detail_id: String,
    pub first_name: String,
    pub last_name: String,
    pub aadhar_number: String,
    pub photo_url: String,
    pub contact_number: String,
    pub contact_type: ContactTypes,
    pub email: String,
}
