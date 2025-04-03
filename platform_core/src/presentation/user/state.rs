use serde::Serialize;
use ts_rs::TS;

#[derive(Serialize, TS)]
#[ts(export, export_to = "../../frontend/types/_gen/GreetState.ts")]
pub struct GreetState {
    pub message: String,
}
