use serde::Serialize;
use ts_rs::TS;

#[derive(Serialize, TS)]
#[ts(export, export_to = "../../frontend/types/_gen/Command.ts")]
pub enum Command {
    Greet,
    FetchUser,
    // Add other commands here
}
