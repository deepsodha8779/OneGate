use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq, Eq, TS)]
#[ts(export, export_to = "bindings/common/Role.ts")]
pub enum Role {
    Admin,
    #[default]
    Guest,
    ServiceProvider,
    Security,
    Member,
}

impl Role {
    pub fn _as_str(&self) -> &'static str {
        match self {
            Role::Admin => "admin",
            Role::Member => "member",
            Role::Guest => "guest",
            Role::ServiceProvider => "serviceProvider",
            Role::Security => "security",
        }
    }
}
