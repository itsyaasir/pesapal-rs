mod auth;
pub mod refund;
pub mod submit_order;

use reqwest::Client as HttpClient;
use serde_json::json;
pub use submit_order::BillingAddress;

use self::refund::{Refund, RefundBuilder};
use self::submit_order::{SubmitOrder, SubmitOrderBuilder};
use crate::environment::Environment;
use crate::error::{PesaPalError, PesaPalResult};
use crate::pesapal::auth::AuthenticationResponse;

/// PesaPal package version
static PESAPAL_PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// [PesaPal] This is the client struct which allows communication with
/// the PesaPal services
#[derive(Debug)]
pub struct PesaPal {
    /// Consumer Key - This is provided by the PesaPal
    consumer_key: String,
    /// Consumer Secret - This is provided by the PesaPal
    consumer_secret: String,
    /// Environment which we are executing the PesaPal Services
    ///
    /// It can be either [Environment::Production] or [Environment::Sandbox]
    pub(crate) env: Environment,
    /// Reqwest HTTP Client
    pub(crate) http_client: HttpClient,
}

impl<'pesa> PesaPal {
    /// This function construct a new PesaPal Instance
    ///
    /// # Example
    /// ```ignore
    /// let pesapal: PesaPal = Pesapal::new(
    ///       std::env("CONSUMER_KEY"),
    ///       std::env("CONSUMER_SECRET"),
    ///       Environment::Production
    /// );
    /// ```
    pub fn new<S: Into<String>>(consumer_key: S, consumer_secret: S, env: Environment) -> Self {
        let http_client = HttpClient::builder()
            .connect_timeout(std::time::Duration::from_secs(10))
            .user_agent(format!("pesapal-rs @{PESAPAL_PACKAGE_VERSION}"))
            .build()
            .expect("Error building http client");

        Self {
            consumer_key: consumer_key.into(),
            consumer_secret: consumer_secret.into(),
            env,
            http_client,
        }
    }

    /// # Pesapal Authentication
    ///
    /// Generate an access token which is used to authenticate Pesapal
    /// requests.
    ///
    /// The authentication request is done via a `POST` request.
    ///
    /// This access token is valid for a maximum period of 5 minutes. Use this
    /// token (sent as a Bearer Token) to access all other Pesapal API 3.0
    /// endpoints.
    ///
    /// See more [here](https://developer.pesapal.com/how-to-integrate/e-commerce/api-30-json/authentication)
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

    /// # Submit Order Builder
    ///
    /// Creates a [SubmitOrderBuilder] for creating a new payment
    /// request.
    ///
    /// The builder is consumed, and returns a [SubmitOrder]
    /// Which we can successfully send the request to start the payment
    /// processing
    ///
    /// See more [here](https://developer.pesapal.com/how-to-integrate/e-commerce/api-30-json/submitorderrequest)
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use pesapal_rs::pesapal::PesaPal;
    ///
    /// let pesapal: PesaPal = Pesapal::new(
    ///       env::var(consumer_key).unwrap(),
    ///       env::var(consumer_secret),
    ///       Environment::Production
    /// );
    ///
    /// let order = pesapal
    ///     .submit_order()
    ///     .currency("KES")
    ///     .amount(2500)
    ///     .description("Shopping")
    ///     .callback_url("https://example.com")
    ///     .cancellation_url("https://example.com")
    ///     .notification_id("asd-egsf1-fdm-sdfs")
    ///     .billing_address(BillingAddress {
    ///         email_address: Some("john@doe.com".to_string()),
    ///         ..Default::default()
    ///      })
    ///     .branch("Example")
    ///     .redirect_mode(RedirectMode::ParentWindow)
    ///     .build()
    ///     .unwrap();
    ///
    /// let response: SubmitOrderResponse = order.send().await.unwrap();
    ///
    /// ```
    pub fn submit_order(&'pesa self) -> SubmitOrderBuilder {
        SubmitOrder::builder(self)
    }

    /// # Refund Payment Builder
    ///
    /// Creates a [RefundBuilder] for creating a new refund
    /// request.
    ///
    /// The builder is consumed, and returns a [Refund]
    /// Which we can successfully send the request to start the refund
    /// processing
    ///
    /// See more [here](https://developer.pesapal.com/how-to-integrate/e-commerce/api-30-json/refund-request)
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use pesapal_rs::pesapal::PesaPal;
    ///
    ///
    /// let pesapal: PesaPal = Pesapal::new(
    ///       env::var(consumer_key).unwrap(),
    ///       env::var(consumer_secret),
    ///       Environment::Production
    /// );
    ///
    /// let order = pesapal
    ///     .refund()
    ///     .amount(2500)
    ///     .remarks("Service not offered")
    ///     .confirmation_code("AA22BB33CC")
    ///     .username("John Doe")
    ///     .build()
    ///     .unwrap();
    ///
    /// let response: RefundResponse = order.send().await.unwrap();
    ///
    /// ```
    pub fn refund(&'pesa self) -> RefundBuilder {
        Refund::builder(self)
    }
}
