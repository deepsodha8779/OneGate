use serde::{Deserialize, Serialize};
// use validator::Validate;
use ts_rs::TS;
use validator::Validate;

use super::{
    common::user::{User, UserInput},
    society::{Society, SocietyInput},
};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS)]
#[ts(export, export_to = "bindings/guest/Guest.ts ")]
pub struct Guest {
    pub id: String,
    pub flat_name: String,
    pub user: User,
    pub society: Society,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS)]
#[ts(export, export_to = "bindings/GuestInput.ts")]
pub struct GuestInput {
    pub flat_name: String,
    pub user: UserInput,
    pub society: SocietyInput,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, PartialEq, Eq, TS)]
#[ts(export, export_to = "bindings/guest/UpdateGuest.ts")]
pub struct UpdateGuest {
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub id: String,
    pub input: GuestInput,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, PartialEq, Eq, TS)]
#[ts(export, export_to = "bindings/guest/Id.ts")]
pub struct Id {
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub id: String,
}
