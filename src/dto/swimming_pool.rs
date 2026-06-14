use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq, Eq, TS, sqlx::FromRow)]
#[ts(export, export_to = "bindings/swimmingpool/SwimmingPool.ts")]
pub struct SwimmingPool {
    pub id: String,
    pub pool_type: String,
    pub pool_details: String,
    pub rules: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, PartialEq, Eq, TS)]
#[ts(export, export_to = "bindings/swimmingpool/SwimmingPool.ts")]
pub struct SwimmingPoolInput {
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub pool_type: String,
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub pool_details: String,
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub rules: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, PartialEq, Eq, TS)]
#[ts(export, export_to = "bindings/swimmingpool/UpdateSwimmingPool.ts")]
pub struct UpdateSwimmingPool {
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub id: String,
    pub input: SwimmingPoolInput,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, PartialEq, Eq, TS)]
#[ts(export, export_to = "bindings/swimmingpool/Id.ts")]
pub struct Id {
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub id: String,
}
