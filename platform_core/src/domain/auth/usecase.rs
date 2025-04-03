use crate::presentation::auth::event::SignInEvent;
use crate::presentation::auth::state::SignInState;

use super::adapter::AuthAdapter;
use super::repository::AuthRepository;

pub trait AuthUseCase: Send + Sync + 'static {
    fn greet(&self, name: &str) -> String;
    fn signin(
        &self,
        event: SignInEvent,
    ) -> Result<SignInState, Box<dyn std::error::Error>>;
}

pub struct AuthUseCaseImpl {
    auth0_service: Box<dyn AuthAdapter>,
    repo: Box<dyn AuthRepository>,
}

impl AuthUseCaseImpl {
    pub fn new(
        auth0_service: Box<dyn AuthAdapter>,
        repo: Box<dyn AuthRepository>,
    ) -> Self {
        Self {
            auth0_service,
            repo,
        }
    }
}

impl AuthUseCase for AuthUseCaseImpl {
    fn greet(&self, name: &str) -> String {
        format!("Hello, {}! You've been greeted from Rust!", name)
    }

    fn signin(
        &self,
        event: SignInEvent,
    ) -> Result<SignInState, Box<dyn std::error::Error>> {
        // Implement the signin logic here
        Ok(SignInState::Waiting)
    }
}
