//! IPN stands for Instant Payment Notification. When a payment is made against
//! a transaction, Pesapal will trigger an IPN call to the notification URL
//! related to this transaction. This notification URL is usually located on
//! your servers. These notifications allows you to be alerted in real time
//! whenever there is a status change for any transaction.
//! An IPN is particular important as it allows you to be notified incase the
//! following happens:

//! -  Your client gets disconnected after payment due to internet issues
//! -  Your client experiences server errors hence Pesapal and your application
//! gets disconnected before callback URL is loaded.
//! - Your client exits your application / closes the browser during payment.
//! - The transaction is rejected.
//!
//! As such, it's mandatory to have IPN configured to allow Pesapal to notify
//! your servers when a status changes. It's also important to note that this
//! IPN URL must be publicly available. In cases where you have strict server
//! rules preventing external systems reaching your end, you must then
//! whitelist all calls from our domain (pesapal.com). Please be informed that
//! IP whitelisting is not feasible as our IP may change without notice.

//! - Before sending Submit Order Requests to Pesapal API 3.0, you are expected
//! to register your IPN URL. Upon registration, you receive a notification Id
//! which is a mandatory field when submitting an order request to Pesapal API
//! 3.0.
//! This notification_id uniquely identifies the endpoint Pesapal will send
//! alerts to whenever a payment status changes for each transaction processed
//! via API 3.0

use crate::{PesaPal, PesaPalError, PesaPalErrorResponse, PesaPalResult};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_aux::prelude::{deserialize_default_from_null, deserialize_number_from_string};

const REGISTER_IPN_URL: &str = "api/URLSetup/RegisterIPN";

#[derive(Debug, Serialize)]
pub struct RegisterIPNRequest {
    /// The notification URL Pesapal will send a status alert to
    pub url: String,
    /// This is the http request method Pesapal will use when triggering the
    /// PIN alert
    pub ipn_notification_type: NotificationType,
}

impl From<RegisterIPN<'_>> for RegisterIPNRequest {
    fn from(value: RegisterIPN) -> Self {
        Self {
            url: value.url,
            ipn_notification_type: value.ipn_notification_type,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum NotificationType {
    Get,
    Post,
}

impl TryFrom<&str> for NotificationType {
    type Error = PesaPalError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "get" => Ok(Self::Get),
            "post" => Ok(Self::Post),
            _ => Err(PesaPalError::Internal(
                "could not parse {value} to notification type".to_string(),
            )),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct RegisterIPNResponse {
    /// The notification url Pesapal will send a status alert to
    pub url: String,
    /// Date and time the IPN URL was registered
    /// It is in UTC
    pub created_date: String,
    /// A unique identifier that's linked to he IPN endpoint URL
    pub ipn_id: String,
    /// Response code
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub status: u16,
    #[serde(deserialize_with = "deserialize_default_from_null")]
    /// Error response
    pub error: Option<PesaPalErrorResponse>,
}

#[derive(Debug, Builder)]
pub struct RegisterIPN<'pesa> {
    #[builder(pattern = "owned")]
    client: &'pesa PesaPal,
    #[builder(setter(into))]
    #[doc = "THe notification URL pesapal will send status alert to"]
    url: String,
    #[builder(try_setter, setter(into))]
    ipn_notification_type: NotificationType,
}

impl RegisterIPN<'_> {
    /// Creates an instance RegisterIPNBuilder
    pub(crate) fn builder(client: &PesaPal) -> RegisterIPNBuilder {
        RegisterIPNBuilder::default().client(client)
    }

    /// # Register IPN URL
    ///
    /// Registers IPN URL
    ///
    /// ## Returns
    ///
    /// [RegisterIPNResponse] - Contains the necessary information about the
    /// IPN URL that has been registered
    ///
    /// ## Errors
    ///
    /// [RegisterIPNError] - Incase the registration fails
    pub async fn send(self) -> PesaPalResult<RegisterIPNResponse> {
        let url = format!("{}/{REGISTER_IPN_URL}", self.client.env.base_url());

        let response = self
            .client
            .http_client
            .post(url)
            .bearer_auth(&self.client.authenticate().await?.token)
            .json::<RegisterIPNRequest>(&self.into())
            .send()
            .await?;

        let res: RegisterIPNResponse = response.json().await?;

        if let Some(error) = res.error {
            return Err(PesaPalError::RegisterIPNError(error));
        }

        Ok(res)
    }
}
