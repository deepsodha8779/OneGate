use serde_json::Value;
use uuid::Uuid;

use crate::{app_state::AppState, dto::staff::Staff, method::convention::ErrorData};

pub async fn get_staff_data(_data: AppState) -> Result<Value, ErrorData> {
    serde_json::to_value(vec![
        Staff
         { id: Uuid::new_v4().to_string(),
           full_name: "Rakesh Modi".to_string(),
           contact: "1234567890".to_string(),
           adharcard_no:  "5678 9012 3455".to_string(),
           photo:format!(
            "https://loremflickr.com/320/240/people")
          },
          Staff {
        id: Uuid::new_v4().to_string(),
        full_name: "Rahul Shrivastav".to_string(),
        contact: "1234567890".to_string(),
        adharcard_no: "5645 9013 3285".to_string(),
        photo: format!(
            "https://loremflickr.com/320/240/people") },
            Staff {
        id: Uuid::new_v4().to_string(),
        full_name: "Deven Iyer".to_string(),
        contact: "6789342011".to_string(),
        adharcard_no: "5645 9013 3285".to_string(),
        photo: "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcSidXCesxXdXZussGr2PDoRb42lBIsiJrZH6ehGw_RqBoJl8y3K_ipuGYzyr5L9bi5Yd2w&usqp=CAU".to_string(), 
    }])
    .map_err(ErrorData::from)
}
