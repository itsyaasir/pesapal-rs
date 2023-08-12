mod auth;
use crate::error::{PesaPalError, PesaPalResult};
use crate::types::{AuthenticationResponse, PesaPalErrorResponse};
use crate::{handle_api_response, ApiEnvironment};

use reqwest::Client as HttpClient;
use serde_json::json;
static CARGO_PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct PesaPal<Env: ApiEnvironment> {
    consumer_key: String,
    consumer_secret: String,
    pub env: Env,
    pub http_client: HttpClient,
}

impl<Env: ApiEnvironment> PesaPal<Env> {
    pub fn new<S: Into<String>>(consumer_key: S, consumer_secret: S, env: Env) -> Self {
        let http_client = HttpClient::builder()
            .connect_timeout(std::time::Duration::from_millis(10_000))
            .user_agent(format!("pesapal-rs @{CARGO_PACKAGE_VERSION}"))
            .build()
            .expect("Error building http client");
        Self {
            consumer_key: consumer_key.into(),
            consumer_secret: consumer_secret.into(),
            env,
            http_client,
        }
    }

    pub async fn authenticate(&self) -> PesaPalResult<AuthenticationResponse> {
        let url = format!("{}/pesapalv3/api/Auth/RequestToken", self.env.base_url());
        let payload = json!({
            "consumer_key":self.consumer_key,
            "consumer_secret": self.consumer_secret
        });

        let response = self
            .http_client
            .post(url)
            .json(&payload)
            .send()
            .await?
            .json()
            .await;

        match response {
            Ok(res) => {
                
            }
            Err(err) => todo!(),
        }

        result
    }
}
