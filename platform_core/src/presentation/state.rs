use serde::Serialize;
use ts_rs::TS;

#[derive(Serialize, TS)]
#[ts(export, export_to="../../frontend/types/_gen/LoginUserInfoState.ts")]
pub struct LoginUserInfoState {
    pub user_id: String,
    pub user_name: String,
    pub user_email: String,
    pub user_role: String,
    pub user_token: String,
    pub user_is_logged_in: bool,
}

#[derive(Serialize, TS)]
#[ts(export, export_to = "../../frontend/types/_gen/GreetState.ts")]
pub struct GreetState {
    pub message: String,
}
