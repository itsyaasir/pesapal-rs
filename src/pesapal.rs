mod auth;
pub mod list_ipn;
pub mod refund;
pub mod register_ipn;
pub mod submit_order;
pub mod transaction_status;

use reqwest::Client as HttpClient;
use serde_json::json;
pub use submit_order::BillingAddress;

use self::list_ipn::ListIPN;
use self::refund::{Refund, RefundBuilder};
use self::register_ipn::{RegisterIPN, RegisterIPNBuilder};
use self::submit_order::{SubmitOrder, SubmitOrderBuilder};
use self::transaction_status::{TransactionStatus, TransactionStatusBuilder};
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

impl PesaPal {
    /// This function construct a new PesaPal Instance
    ///
    /// # Example
    /// ```ignore
    /// let pesapal: PesaPal = Pesapal::new(
    ///       std::env("CONSUMER_KEY").unwrap(),
    ///       std::env("CONSUMER_SECRET").unwrap(),
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
    ///       env::var(consumer_secret).unwrap(),
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
    pub fn submit_order(&self) -> SubmitOrderBuilder {
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
    ///       env::var(consumer_secret).unwrap(),
    ///       Environment::Production
    /// );
    ///
    /// let refund_order = pesapal
    ///     .refund()
    ///     .amount(2500)
    ///     .remarks("Service not offered")
    ///     .confirmation_code("AA22BB33CC")
    ///     .username("John Doe")
    ///     .build()
    ///     .unwrap();
    ///
    /// let response: RefundResponse = refund_order.send().await.unwrap();
    ///
    /// ```
    pub fn refund(&self) -> RefundBuilder {
        Refund::builder(self)
    }

    /// Register IPN URL builder
    ///
    /// Creates a [RegisterIPNBuilder] which is used for registering URL which
    /// Pesapal will send notification about the payment in real-time.
    ///
    /// When a payment is made against a transaction, Pesapal will trigger an
    /// IPN call to the notification URL related to this transaction
    ///
    /// The notification allows you to be alerted in real-time
    ///
    /// The builder is consumed and returns a [RegisterIPN]
    /// which can successfully start the registration of the IPN
    /// URL
    /// See more [here](https://developer.pesapal.com/how-to-integrate/e-commerce/api-30-json/registeripnurl)
    ///
    /// # Example
    ///
    /// ``` ignore
    ///
    /// use crate::pesapal::PesaPal;
    ///
    /// let pesapal: PesaPal = Pesapal::new(
    ///       env::var(consumer_key).unwrap(),
    ///       env::var(consumer_secret).unwrap(),
    ///       Environment::Production
    /// );
    ///
    /// let register_ipn_response = pesapal
    ///     .register_ipn_url()
    ///     .url("https://example.com")
    ///     .ipn_notification_type("GET")
    ///     .build()
    ///     .unwrap();
    ///
    /// let response: RegisterIPNResponse = register_ipn_response.send().await.
    /// unwrap();
    pub fn register_ipn_url(&self) -> RegisterIPNBuilder {
        RegisterIPN::builder(self)
    }

    /// List IPN URL builder
    ///
    /// Creates a [ListIPN] which is used for listing all the IPN URLs
    /// registered for the merchant.
    ///
    /// The builder is consumed and returns a [ListIPN]
    /// which can successfully start the listing of the IPN
    /// URLs
    ///
    /// See more [here](https://developer.pesapal.com/how-to-integrate/e-commerce/api-30-json/getregisteredipn)
    ///
    /// # Example
    ///
    /// ``` ignore
    ///
    /// use crate::pesapal::PesaPal;
    ///
    /// let pesapal: PesaPal = Pesapal::new(
    ///      env::var(consumer_key).unwrap(),
    ///      env::var(consumer_secret).unwrap(),
    ///      Environment::Production
    /// );
    ///
    /// let list_ipn_response: IPNListResponse = pesapal
    ///    .list_ipn_urls()
    ///    .send()
    ///    .await
    ///    .unwrap();
    ///
    /// ```
    pub fn list_ipn_urls(&self) -> ListIPN {
        ListIPN::new(self)
    }

    /// Transaction Status builder
    ///
    /// Creates a [TransactionStatusBuilder] which is used for checking the
    /// status of a transaction
    ///
    /// The builder is consumed and returns a [TransactionStatus]
    /// which can successfully start the checking of the transaction status
    ///
    /// See more [here](https://developer.pesapal.com/how-to-integrate/e-commerce/api-30-json/gettransactionstatus)
    ///
    /// # Example
    ///
    /// ``` ignore
    ///
    /// use crate::pesapal::PesaPal;
    ///
    /// let pesapal: PesaPal = Pesapal::new(
    ///     env::var(consumer_key).unwrap(),
    ///     env::var(consumer_secret).unwrap(),
    ///     Environment::Production
    /// );
    ///
    /// let transaction_status_response: TransactionStatusResponse = pesapal
    ///    .transaction_status()
    ///    .order_tracking_id("asdasd")
    ///     .build()
    ///     .unwrap()
    ///     .send()
    ///     .await
    ///     .unwrap();
    ///
    /// ```
    pub fn transaction_status(&self) -> TransactionStatusBuilder {
        TransactionStatus::builder(self)
    }
}
