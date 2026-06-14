use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::contact::ContactTypes;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS, sqlx::FromRow, sqlx::Type)]
pub struct UserDetail {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub aadhar_number: String,
    pub photo_url: String,
    pub contact_number: String,
    pub contact_type: ContactTypes,
    pub email: String,
}
