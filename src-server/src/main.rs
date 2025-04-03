use axum::{
    extract::{Query, State},
    response::IntoResponse,
    routing::get,
    Router,
};
use serde::Deserialize;
use std::sync::Arc;
use tower_http::cors;
use tracing;
use tracing_subscriber;
use zeroize::Zeroizing;

use platform_core::core::credential::PlatformCredentialManager;
use platform_core::infrastructure::auth::auth0_service::Auth0Service;
use platform_core::infrastructure::auth::db_repository::AuthRepositoryImpl;
use platform_core::infrastructure::auth::db_resource::AlloyDbResource;
use platform_core::{
    core::credential::CredentialManager, domain::auth::usecase::AuthUseCaseImpl,
};

#[derive(Debug, Deserialize)]
struct GreetParams {
    name: String,
}

async fn greet_handler(
    State(context): State<AppContext>,
    Query(params): Query<GreetParams>,
) -> impl IntoResponse {
    tracing::info!("Greet handler called with params: {:?}", params);
    let message = context.auth_usecase.greet(&params.name);
    message
}

#[derive(Clone)]
pub struct AppContext {
    credential_manager: Arc<dyn CredentialManager>,
    auth_usecase: Arc<dyn platform_core::domain::auth::usecase::AuthUseCase>,
}

impl AppContext {
    pub fn inject(env_key: &str) -> Self {
        match env_key {
            "dev" => {
                tracing::info!("Injecting dev dependencies");

                AppContext {
                    credential_manager: Arc::new(
                        PlatformCredentialManager::new(Zeroizing::new(
                            "dev_credential".into(),
                        )),
                    ),
                    auth_usecase: Arc::new(AuthUseCaseImpl::new(
                        Box::new(Auth0Service::new(
                            "https://dev-auth.example.com".into(),
                            "dev_client_id".into(),
                            "dev_secret".into(),
                        )),
                        Box::new(AuthRepositoryImpl::new(Box::new(
                            AlloyDbResource::new(),
                        ))),
                    )),
                }
            }
            "prod" => {
                tracing::info!("Injecting prod dependencies");

                AppContext {
                    credential_manager: Arc::new(
                        PlatformCredentialManager::new(Zeroizing::new(
                            "prod_credential".into(),
                        )),
                    ),
                    auth_usecase: Arc::new(AuthUseCaseImpl::new(
                        Box::new(Auth0Service::new(
                            "https://prod-auth.example.com".into(),
                            "prod_client_id".into(),
                            "prod_secret".into(),
                        )),
                        Box::new(AuthRepositoryImpl::new(Box::new(
                            AlloyDbResource::new(),
                        ))),
                    )),
                }
            }
            _ => panic!("Unknown environment key: {env_key}"),
        }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    tracing::info!("Starting server");

    let context = AppContext::inject("dev");

    let cors = cors::CorsLayer::permissive();

    let app = Router::new()
        .route("/greet", get(greet_handler))
        .with_state(context)
        .layer(cors);

    println!("Server running");
    let listener = tokio::net::TcpListener::bind("localhost:8080")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
