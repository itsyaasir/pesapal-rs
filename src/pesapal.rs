mod auth;
mod submit_order;

use reqwest::Client as HttpClient;
use serde_json::json;
pub use submit_order::BillingAddress;

use self::submit_order::{SubmitOrder, SubmitOrderBuilder};
use crate::environment::Environment;
use crate::error::{PesaPalError, PesaPalResult};
use crate::pesapal::auth::AuthenticationResponse;

static CARGO_PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Clone, Default)]
pub struct PesaPal {
    consumer_key: String,
    consumer_secret: String,
    pub(crate) env: Environment,
    pub(crate) http_client: HttpClient,
}

impl<'pesa> PesaPal {
    pub fn new<S: Into<String>>(consumer_key: S, consumer_secret: S, env: Environment) -> Self {
        let http_client = HttpClient::builder()
            .connect_timeout(std::time::Duration::from_secs(10))
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
        let url = format!("{}/api/Auth/RequestToken", self.env.base_url());
        let payload = json!({
            "consumer_key":self.consumer_key,
            "consumer_secret": self.consumer_secret
        });

        let response = self.http_client.post(url).json(&payload).send().await?;

        if response.status().is_success() {
            let value = response.json::<_>().await?;
            return Ok(value);
        }

        let err = response.json().await?;
        Err(PesaPalError::AuthenticationError(err))
    }

    pub fn submit_order(&'pesa self) -> SubmitOrderBuilder {
        SubmitOrder::builder(self)
    }
}
