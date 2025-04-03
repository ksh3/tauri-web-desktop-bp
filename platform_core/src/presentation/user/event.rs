use serde::Serialize;
use ts_rs::TS;

#[derive(Serialize, TS)]
#[ts(export, export_to = "../../frontend/types/_gen/GreetEvent.ts")]
pub struct GreetEvent {
    pub name: String,
}
