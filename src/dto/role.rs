use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::common::user::User;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS)]
#[ts(export, export_to = "bindings/SuperAdmin.ts")]
pub struct SuperAdmin {
    pub user: User,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS)]
#[ts(export, export_to = "bindings/Admin.ts")]
pub struct Admin {
    pub user: User,
}
