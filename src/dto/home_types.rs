use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

use super::society::Society;

#[derive(Deserialize, Serialize, Debug, Clone, Validate, PartialEq, Eq, TS)]
#[ts(export, export_to = "bindings/home/HomeInput.ts")]
pub struct HomeInput {
    pub wing: String,
    pub no: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, PartialEq, Eq, TS)]
#[ts(export, export_to = "bindings/home/UpdateHome.ts")]
pub struct UpdateHome {
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub id: String,
    pub input: HomeInput,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, PartialEq, Eq, TS)]
#[ts(export, export_to = "bindings/home/Id.ts")]
pub struct Id {
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS, sqlx::FromRow)]
#[ts(export, export_to = "bindings/home/Home.ts")]
pub struct Home {
    pub id: String,
    pub society: Society,
    pub wing: String,
    pub no: i32,
}
