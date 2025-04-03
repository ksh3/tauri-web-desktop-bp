use super::db_resource::AuthResource;
use crate::domain::auth::repository::AuthRepository;

pub struct AuthRepositoryImpl {
    resource: Box<dyn AuthResource>,
}

impl AuthRepositoryImpl {
    pub fn new(resource: Box<dyn AuthResource>) -> Self {
        Self { resource }
    }
}

impl AuthRepository for AuthRepositoryImpl {}
