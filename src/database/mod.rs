pub mod address;
pub mod amenity;
pub mod auth;
pub mod complaint;
pub mod contact;
pub mod home;
pub mod maid;
pub mod member;
pub mod notice;
pub mod role;
pub mod security;
pub mod security_guest;
pub mod service_provider;
pub mod society;
pub mod user;
pub mod user_details;

// use chrono::{DateTime, Local};
// struct Slot {
//     id: i32,
//     range: String,
//     amenity_id: i32,
// }

// struct AmenityServiceProvider {
//     user_id: i32,
//     amenity_id: i32,
// }

// struct AmenityRole {
//     role: String,
//     amenity_id: i32,
// }

// struct EntryLog {
//     id: i32,
//     is_delete: bool,
//     society_id: i32, //null
//     amenity_id: i32, //null
//     flat_id: i32,    //null
//     // user : user_id,
//     enter_date_time: DateTime<Local>,
//     exit_date_time: Option<DateTime<Local>>, // nullable
// }

// struct AmenityGuest {
//     amenity_id: i32,
//     guest_id: i32,
//     start_date_time: DateTime<Local>,
//     end_date_time: DateTime<Local>,
//     qr_code: String,
// }

// struct Guest {
//     id: i32,
//     user_id: i32,
// }

// struct Item {
//     id: i32,
//     name: String,
// }

// struct List {
//     item: Vec<Item>,
// }

// struct ItemDB {
//     id: i32,
//     name: String,
//     list_id: i32,
// }

// struct ListDB {
//     id: i32,
// }
