use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::society::Society;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS, sqlx::FromRow)]
#[ts(export, export_to = "bindings/complaint/Complaint.ts ")]
pub struct Complaint {
    pub id: String,
    pub complaint_by: String,
    pub complaint_title: String,
    pub complaint_description: String,
    pub status: String,
    pub society: Society,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS, sqlx::FromRow)]
#[ts(export, export_to = "bindings/ComplaintInput.ts")]
pub struct ComplaintInput {
    pub complaint_by: String,
    pub complaint_title: String,
    pub complaint_description: String,
    pub status: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS, sqlx::FromRow)]
#[ts(export, export_to = "bindings/complaint/UpdateComplaint.ts")]
pub struct UpdateComplaint {
    pub id: String,
    pub input: ComplaintInput,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS, sqlx::FromRow)]
#[ts(export, export_to = "bindings/complaint/Id.ts")]
pub struct Id {
    pub id: String,
}
