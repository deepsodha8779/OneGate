use serde::{Deserialize, Serialize};
use sqlx;
use ts_rs::TS;

use super::contact::ContactTypes;
use crate::dto::service_provider::ServiceProviderTypes;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS, sqlx::FromRow)]
#[ts(export, export_to = "bindings/service_provider/ServiceProviderData.ts")]
pub struct ServiceProviderData {
    pub id: String,
    pub service_provider_types: ServiceProviderTypes,
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
