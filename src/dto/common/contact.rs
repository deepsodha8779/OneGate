use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS, sqlx::Type)]
#[ts(export, export_to = "bindings/common/ContactType.ts")]
pub enum ContactType {
    Mobile,
    Home,
    Work,
}

impl ContactType {
    pub fn _as_str(&self) -> &'static str {
        match self {
            ContactType::Mobile => "Mobile",
            ContactType::Home => "Home",
            ContactType::Work => "Work",
        }
    }
}
