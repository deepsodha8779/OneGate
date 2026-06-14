use super::{entrylog::EntryLog, maintenance::FlatMaintenance, member::Member};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Deserialize, Serialize, Clone, Debug, TS)]
#[ts(export, export_to = "bindings/Flat.ts")]
pub struct Flat {
    pub id: i32,
    pub flat_no: String,
    pub floor_no: i8,
    pub wing: String,
    pub maintenance: FlatMaintenance,
    pub members: Vec<Member>,
    pub entries: Vec<EntryLog>,
}
