use async_trait::async_trait;
use serde_json::json;

use crate::domain::auth::adapter::AuthAdapter;

pub struct Auth0Service {
    client: reqwest::Client,
    domain: String,
    client_id: String,
    client_secret: String,
}

impl Auth0Service {
    pub fn new(
        domain: String,
        client_id: String,
        client_secret: String,
    ) -> Self {
        Self {
            client: reqwest::Client::new(),
            domain,
            client_id,
            client_secret,
        }
    }
}

#[async_trait]
impl AuthAdapter for Auth0Service {
    async fn signin(
        &self,
        email: &str,
        password: &str,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!("https://{}/oauth/token", self.domain);
        let body = json!({
            "client_id": self.client_id,
            "client_secret": self.client_secret,
            "audience": format!("https://{}/api/v2/", self.domain),
            "grant_type": "password",
            "username": email,
            "password": password,
            "scope": "openid",
        });

        let res = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .body(body.to_string())
            .send()
            .await?;

        match res.status() {
            reqwest::StatusCode::OK => {
                let res_body: serde_json::Value = res.json().await?;
                return Ok(res_body);
            }
            _other => {
                let error_message = format!(
                    "Error: {} - {}",
                    res.status(),
                    res.text().await.unwrap_or_default()
                );
                return Err(error_message.into());
            }
        }
    }

    async fn signup_with_tenancy(
        &self,
        email: &str,
        password: &str,
        tenancy_id: &str,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!("https://{}/dbconnections/signup", self.domain);
        let body = json!({
            "client_id": self.client_id,
            "email": email,
            "password": password,
            "connection": "Username-Password-Authentication",
            "user_metadata": {
                "tenancy_id": tenancy_id,
            },
        });

        let res = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .body(body.to_string())
            .send()
            .await?;

        let res_body: serde_json::Value = res.json().await?;

        Ok(res_body)
    }

    async fn get_access_token(
        &self,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!("https://{}/oauth/token", self.domain);
        let body = json!({
            "client_id": self.client_id,
            "client_secret": self.client_secret,
            "audience": format!("https://{}/api/v2/", self.domain),
            "grant_type": "client_credentials",
        });

        let res = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .body(body.to_string())
            .send()
            .await?;

        let res_body: serde_json::Value = res.json().await?;
        let access_token =
            res_body["access_token"].as_str().unwrap().to_string();

        Ok(access_token)
    }

    async fn get_user_info(
        &self,
        access_token: &str,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!("https://{}/userinfo", self.domain);

        let res = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?;

        let res_body: serde_json::Value = res.json().await?;

        Ok(res_body)
    }
}
