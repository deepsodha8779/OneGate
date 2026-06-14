use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;
#[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq, Eq, TS)]
#[ts(export, export_to = "bindings/common/Address.ts")]
pub struct Address {
    pub pin_code: String,
    pub city: String,
    pub state: String,
    pub country: String,
    pub address_line1: String,
    pub address_line2: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, PartialEq, Eq, TS)]
#[ts(export, export_to = "bindings/common/AddressInput.ts")]
pub struct AddressInput {
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub pin_code: String,
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub city: String,
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub state: String,
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub address_line1: String,
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub country: String,
    pub address_line2: String,
}
