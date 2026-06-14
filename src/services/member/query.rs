use crate::{
    app_state::AppState,
    dto::{common::user::User, member::Member, user_details::UserDetail},
    method::convention::ErrorData,
};
use anyhow::Result;
use serde_json::Value;
use uuid::Uuid;

pub async fn get_member_data(_data: AppState) -> Result<Value, ErrorData> {
    serde_json::to_value(vec![
        Member {
            id: Uuid::new_v4().as_simple().to_string(),
            user: User {
                id: Uuid::new_v4().as_simple().to_string(),
                user_details: UserDetail {
                    id: Uuid::new_v4().as_simple().to_string(),
                    first_name: "Ri".to_string(),
                    last_name: "Ya".to_string(),
                    aadhar_number: "1234567890".to_string(),
                    contact_type: crate::database::contact::ContactTypes::Home,
                    photo_url: "W.png".to_string(),
                    contact_number: "2345678910".to_string(),
                    email: "ryt@gmsil.com".to_string(),
                },
                is_block: false,
                is_deleted: false,
            },
        },
        Member {
            id: Uuid::new_v4().as_simple().to_string(),
            user: User {
                id: Uuid::new_v4().as_simple().to_string(),
                user_details: UserDetail {
                    id: Uuid::new_v4().as_simple().to_string(),
                    first_name: "Aarti".to_string(),
                    last_name: "Sharma".to_string(),
                    aadhar_number: "123423456".to_string(),
                    contact_type: crate::database::contact::ContactTypes::Home,
                    photo_url: "Wvyt.png".to_string(),
                    contact_number: "2323445610".to_string(),
                    email: "rytwng@gmsil.com".to_string(),
                },
                is_block: false,
                is_deleted: false,
            },
        },
    ])
    .map_err(ErrorData::from)
}
