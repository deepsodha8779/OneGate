use serde::{Deserialize, Serialize};
use sqlx;
use ts_rs::TS;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS, sqlx::FromRow)]
pub struct Contact {
    user_id: String,
    contact_type: ContactTypes,
    value: String,
}

#[derive(Serialize, Deserialize, Debug, sqlx::Type, Clone, TS, PartialEq, Eq)]
#[sqlx(type_name = "contact_types", rename_all = "lowercase")]
#[ts(export, export_to = "bindings/common/contact/ContactTypes.ts")]
pub enum ContactTypes {
    Mobile,
    Home,
    Work,
}
