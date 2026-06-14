use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;
// This is for Super Admin only
// Can be done by admin only
#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, PartialEq, Eq, TS)]
#[ts(export, export_to = "bindings/PaymentPlan.ts")]
pub struct Payment {
    pub id: String,
    pub name: String,
    pub plan: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, PartialEq, Eq, TS)]
#[ts(export, export_to = "bindings/Payment.ts")]
pub struct PaymentInput {
    pub plan: String,
    pub name: String, // Store json coming from payment provider
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, PartialEq, Eq, TS)]
#[ts(export, export_to = "bindings/maid/UpdatePayment.ts")]
pub struct UpdatePayment {
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub id: String,
    pub input: PaymentInput,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, PartialEq, Eq, TS)]
#[ts(export, export_to = "bindings/payment/Id.ts")]
pub struct Id {
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub id: String,
}
