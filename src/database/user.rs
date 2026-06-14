use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::contact::ContactTypes;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS, sqlx::FromRow)]
#[ts(export, export_to = "bindings/common/user/UserData.ts")]
pub struct UserData {
    pub id: String,
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
