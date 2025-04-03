use async_trait::async_trait;

#[async_trait]
pub trait AuthAdapter: Send + Sync + 'static {
    async fn signin(
        &self,
        email: &str,
        password: &str,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>>;
    async fn signup_with_tenancy(
        &self,
        email: &str,
        password: &str,
        tenancy_id: &str,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>>;
    async fn get_access_token(
        &self,
    ) -> Result<String, Box<dyn std::error::Error>>;
    async fn get_user_info(
        &self,
        access_token: &str,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>>;
}
