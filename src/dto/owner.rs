// use validator::Validate;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

// use super::common::user::User;

#[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq, Eq, TS, sqlx::FromRow)]
#[ts(export, export_to = "bindings/Owner.ts")]
pub struct Owner {
    pub id: String,
    pub name: String,
    pub floor_no: String,
    pub flat_no: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq, Eq, TS, sqlx::FromRow)]
#[ts(export, export_to = "bindings/owner/OwnerInput.ts")]
pub struct OwnerInput {
    pub name: String,
    pub floor_no: String,
    pub flat_no: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, PartialEq, Eq, TS)]
#[ts(export, export_to = "bindings/owner/UpdateOwner.ts")]
pub struct UpdateOwner {
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub id: String,
    pub input: OwnerInput,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, PartialEq, Eq, TS)]
#[ts(export, export_to = "bindings/owner/Id.ts")]
pub struct Id {
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub id: String,
}
