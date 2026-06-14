use crate::{
    database::contact::ContactTypes,
    dto::common::user::{User, UserInput},
};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS, sqlx::FromRow)]
#[ts(export, export_to = "bindings/common/ContactInput.ts")]
pub struct ContactInput {
    pub user: UserInput,
    pub contact: ContactTypes,
    pub value: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS, sqlx::FromRow)]
#[ts(export, export_to = "bindings/common/Contact.ts")]
pub struct Contact {
    pub id: String,
    pub user: User,
    pub value: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS, sqlx::FromRow)]
#[ts(export, export_to = "bindings/common/contact/ContactInput.ts")]
pub struct UpdateContact {
    pub id: String,
    pub input: ContactInput,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS, sqlx::FromRow)]
#[ts(export, export_to = "bindings/common/contact/ContactInput.ts")]
pub struct Id {
    pub id: String,
}
