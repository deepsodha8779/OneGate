use serde::{Deserialize, Serialize};
// use validator::Validate;
use ts_rs::TS;

#[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq, Eq, TS)]
#[ts(export, export_to = "bindings/Slot.ts")]
pub struct Slot {
    pub range: String,
}
