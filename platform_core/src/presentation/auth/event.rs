use serde::Serialize;
use ts_rs::TS;

#[derive(Serialize, TS)]
#[ts(export, export_to = "../../frontend/types/_gen/SignInEvent.ts")]
pub enum SignInEvent {
    Email { email: String, password: String },
    Google { id_token: String },
}
