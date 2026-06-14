use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

use super::society::Society;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS)]
#[ts(export, export_to = "bindings/notice/Notice.ts ")]
pub struct Notice {
    pub id: String,
    pub title: String,
    pub notice_description: String,
    pub society: Society,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS)]
#[ts(export, export_to = "bindings/NoticeInput.ts")]
pub struct NoticeInput {
    pub title: String,
    pub notice_description: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, PartialEq, Eq, TS)]
#[ts(export, export_to = "bindings/notice/UpdateNotice.ts")]
pub struct UpdateNotice {
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub id: String,
    pub input: NoticeInput,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, PartialEq, Eq, TS)]
#[ts(export, export_to = "bindings/notice/Id.ts")]
pub struct Id {
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub id: String,
}
