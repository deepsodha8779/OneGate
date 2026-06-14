use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::common::user::{User, UserInput};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS, sqlx::FromRow)]
#[ts(export, export_to = "bindings/member/Member.ts")]
pub struct Member {
    pub id: String,
    pub user: User,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS, sqlx::FromRow)]
#[ts(export, export_to = "bindings/member/MemberInput.ts")]
pub struct MemberInput {
    pub user: UserInput,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS, sqlx::FromRow)]
#[ts(export, export_to = "bindings/member/UpdateMember.ts")]
pub struct UpdateMember {
    pub id: String,
    pub input: MemberInput,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS, sqlx::FromRow)]
#[ts(export, export_to = "bindings/member/Id.ts")]
pub struct Id {
    pub id: String,
}
