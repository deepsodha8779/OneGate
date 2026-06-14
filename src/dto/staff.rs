use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq, Eq, TS, sqlx::FromRow)]
#[ts(export, export_to = "bindings/staff/Staff.ts")]
pub struct Staff {
    pub id: String,
    pub full_name: String,
    pub contact: String,
    pub adharcard_no: String,
    pub photo: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, PartialEq, Eq, TS)]
#[ts(export, export_to = "bindings/staff/StaffInput.ts")]
pub struct StaffInput {
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub full_name: String,
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub contact: String,
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub adharcard_no: String,
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub photo: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, PartialEq, Eq, TS)]
#[ts(export, export_to = "bindings/staff/UpdateStaff.ts")]
pub struct UpdateStaff {
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub id: String,
    pub input: StaffInput,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, PartialEq, Eq, TS)]
#[ts(export, export_to = "bindings/staff/Id.ts")]
pub struct Id {
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub id: String,
}
