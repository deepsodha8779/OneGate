use anyhow::Result;

use crate::dto::common::user::User;
use crate::dto::society::Society;
use crate::dto::user_details::UserDetail;
use crate::{
    app_state::AppState,
    dto::{common::address::Address, guest::Guest},
    method::convention::ErrorData,
};

use serde_json::Value;
use uuid::Uuid;
pub async fn get_guest_data(_data: AppState) -> Result<Value, ErrorData> {
    serde_json::to_value(vec![
        Guest {
            id: Uuid::new_v4().to_string(),
            society: Society {
                id: Uuid::new_v4().to_string(),
                name: "Sudarshan Heights".to_string(),
                address: Address {
                    pin_code: "441106".to_string(),
                    city: "Nagpur".to_string(),
                    state: "MH".to_string(),
                    country: "IN".to_string(),
                    address_line1: "kotwal Nagar".to_string(),
                    address_line2: "Nagpur".to_string(),
                },
                allowed_attempts: 4,
                maintenance_per_month: 23000,
            },
            flat_name: "Ghar".to_string(),
            user: User {
                id: Uuid::new_v4().to_string(),
                user_details: UserDetail {
                    id: Uuid::new_v4().to_string(),
                    first_name: "Percy".to_string(),
                    last_name: "Jackson".to_string(),
                    aadhar_number: "1234567890".to_string(),
                    photo_url: "Percy.png".to_string(),
                    contact_type: crate::database::contact::ContactTypes::Home,
                    contact_number: "9876543210".to_string(),
                    email: "percy@gmail.com".to_string(),
                },
                is_block: false,
                is_deleted: false,
            },
        },
        Guest {
            id: Uuid::new_v4().to_string(),
            society: Society {
                id: Uuid::new_v4().to_string(),
                name: "Gandhi Heights".to_string(),
                address: Address {
                    pin_code: "441106".to_string(),
                    city: "Nagpur".to_string(),
                    state: "MH".to_string(),
                    country: "IN".to_string(),
                    address_line1: "kotwal Nagar".to_string(),
                    address_line2: "Nagpur".to_string(),
                },
                allowed_attempts: 4,
                maintenance_per_month: 23000,
            },
            flat_name: "Home".to_string(),
            user: User {
                id: Uuid::new_v4().to_string(),
                user_details: UserDetail {
                    id: Uuid::new_v4().to_string(),
                    first_name: "Joey".to_string(),
                    last_name: "Jackson".to_string(),
                    aadhar_number: "12345678".to_string(),
                    contact_type: crate::database::contact::ContactTypes::Home,
                    photo_url: "joey.png".to_string(),
                    contact_number: "92345789".to_string(),
                    email: "joey@gmail.com".to_string(),
                },
                is_block: false,
                is_deleted: false,
            },
        },
    ])
    .map_err(ErrorData::from)
}
