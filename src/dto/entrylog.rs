// use super::{common::user::{User, UserInput}};

use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

use super::common::user::{User, UserInput};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS, sqlx::FromRow)]
#[ts(export, export_to = "bindings/EntryLog.ts")]
pub struct EntryLog {
    pub id: String,
    pub user: User,
    pub enter_date_time: String,
    pub exit_date_time: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS, sqlx::FromRow)]
#[ts(export, export_to = "bindings/EntryLogInput.ts")]
pub struct EntryLogInput {
    pub user: UserInput,
    pub enter_date_time: String,
    pub exit_date_time: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, PartialEq, Eq, TS)]
#[ts(export, export_to = "bindings/home/UpdateEntryLog.ts")]
pub struct UpdateEntryLog {
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub id: String,
    pub input: EntryLogInput,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, PartialEq, Eq, TS)]
#[ts(export, export_to = "bindings/home/Id.ts")]
pub struct Id {
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub id: String,
}
