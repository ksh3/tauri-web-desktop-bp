use serde::Serialize;
use ts_rs::TS;

#[derive(Serialize, TS)]
#[ts(export, export_to = "../../frontend/types/_gen/GreetState.ts")]
pub struct GreetState {
    pub message: String,
}

#[derive(Serialize, TS)]
#[ts(export, export_to = "../../frontend/types/_gen/SignInEvent.ts")]
pub enum SignInState {
    Success {
        id_token: String,
        access_token: String,
    },
    Failure {
        error: String,
    },
    Waiting,
}
