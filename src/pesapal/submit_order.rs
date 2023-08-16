//! The submit order endpoint allows to create new payment order
//! A good example would be a case where the customer has clicked pay now
//! button on your website. At this point, you call the SubmitOrderRequest and
//! in return you will get a response which contains a payment redirect URL
//! which you then redirect the customer to or load the URL as an iframe within
//! your site in case you don’t want to redirect the customer off your application.

//! The payment URL will contain the payment methods presented to the customer
//! by Pesapal. After the customer has made the payment, they will be
//! redirected to your callback URL which you will have already provided to us
//! as part of submit order request.

use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_aux::prelude::deserialize_default_from_null;
use uuid::Uuid;

use super::PesaPal;
use crate::error::{PesaPalError, PesaPalErrorResponse, PesaPalResult};

const SUBMIT_ORDER_REQUEST_URL: &str = "api/Transactions/SubmitOrderRequest";

/// Submit Order Request
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubmitOrderRequest {
    /// Unique merchant reference
    pub id: String,
    /// Currency which is used to charge the customers
    pub currency: String,
    /// Amount to be processed
    pub amount: u64,
    /// Description of the order
    pub description: String,
    /// Accepts values TOP_WINDOW or PARENT_WINDOW.
    /// If left blank, the default value used will be TOP_WINDOW.
    /// This parameter allows you to define where your callback URL will be
    /// loaded;
    /// * TOP_WINDOW returns to the topmost window in the hierarchy of
    /// windows.
    /// * PARENT_WINDOW returns the immediate parent of a window.
    pub redirect_mode: RedirectMode,
    /// A URL which PesaPal will redirect to process the payment
    pub callback_url: String,
    /// A URL which will redirect the client incase the click on cancel request
    pub cancellation_url: Option<String>,
    /// Represents IPN URL which PesaPal uses to end notification after the
    /// Payments have been processed
    ///
    /// You are required to register all IPN Urls
    pub notification_id: String,
    /// If your business has multiple stores / branches, you can define the
    /// name of the store / branch to which this particular payment will be
    /// accredited to.
    pub branch: Option<String>,
    /// Billing address of the customer
    pub billing_address: BillingAddress,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RedirectMode {
    #[default]
    TopWindow,
    ParentWindow,
}

/// The billing address of a customer
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Default)]
pub struct BillingAddress {
    /// customer's email address.
    ///
    /// This is Optional if phone number has been provided
    /// Otherwise mandatory
    ///
    /// The BillingAddress will fail if this is not met
    pub email_address: Option<String>,
    /// Customer's Phone Number
    ///
    /// This is optional if email address has been provided,
    /// Otherwise mandatory
    ///
    pub phone_number: Option<u64>,
    /// country code in [ISO-3166-1]
    ///
    /// It is usually two characters long
    ///
    /// https://en.wikipedia.org/wiki/ISO_3166-1
    pub country_code: Option<String>,
    /// Customer's first name
    pub first_name: Option<String>,
    /// Customer's middle name
    pub middle_name: Option<String>,
    /// Customer's last name
    pub last_name: Option<String>,
    // Main Address
    pub line_1: Option<String>,
    // Alternative Address
    pub line_2: Option<String>,
    /// Customer's City
    pub city: Option<String>,
    /// Customer's state
    ///
    /// Maximum: three character long
    pub state: Option<String>,
    // Customer's postal code
    pub postal_code: Option<String>,
    /// Customer's zip code
    pub zip_code: Option<String>,
}

impl BillingAddress {
    /// Create a new billing address
    ///
    /// Either the phone number or the email must be provided,
    /// otherwise the request will fail.
    ///
    pub fn new(&self, phone_number: Option<u64>, email: Option<String>) -> PesaPalResult<Self> {
        if phone_number.is_none() && email.is_none() {
            return Err(PesaPalError::ValidationError(
                "either email or password need to be provided".to_string(),
            ));
        }

        Ok(Self {
            email_address: email,
            phone_number,
            ..Default::default()
        })
    }
}

impl From<SubmitOrder<'_>> for SubmitOrderRequest {
    fn from(value: SubmitOrder) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            currency: value.currency,
            amount: value.amount,
            description: value.description,
            redirect_mode: value.redirect_mode,
            callback_url: value.callback_url,
            notification_id: value.notification_id,
            cancellation_url: value.cancellation_url,
            branch: value.branch,
            billing_address: value.billing_address,
        }
    }
}

/// The Submit Order response after a payment has been created successfully
#[derive(Default, Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct SubmitOrderResponse {
    /// Unique order id generated by PesaPal
    pub order_tracking_id: String,
    /// Unique ID received as part of the [SubmitOrderRequest]
    pub merchant_reference: String,
    /// URL generated that contains the payment instructions.
    ///
    /// Redirect to this URl or load it within an iframe
    pub redirect_url: String,
    /// Error message
    #[serde(deserialize_with = "deserialize_default_from_null")]
    pub error: Option<PesaPalErrorResponse>,
    /// Response Message
    pub status: String,
}

/// This is the submit order builder
#[derive(Builder, Debug, Clone)]
#[builder(build_fn(validate = "Self::validate"))]
pub struct SubmitOrder<'pesa> {
    #[builder(pattern = "owned")]
    client: &'pesa PesaPal,
    #[builder(setter(into))]
    #[doc = r"Currency which is used to charge the customers"]
    currency: String,
    #[doc = r"Amount to be processed"]
    amount: u64,
    #[builder(setter(into))]
    #[doc = r"Description of the order"]
    description: String,
    #[builder(default)]
    #[doc = r"Accepts values TOP_WINDOW or PARENT_WINDOW.
     If left blank, the default value used will be TOP_WINDOW.
     This parameter allows you to define where your callback URL will be
     loaded;
     - TOP_WINDOW returns to the topmost window in the hierarchy of
     windows.
     - PARENT_WINDOW returns the immediate parent of a window."]
    redirect_mode: RedirectMode,
    #[builder(setter(into))]
    #[doc = r"URL which PesaPal will re-direct for the payment processing"]
    callback_url: String,
    #[builder(setter(into, strip_option), default)]
    #[doc = r"A valid URL which PesaPal will redirect client incase
    they cancel the payment"]
    cancellation_url: Option<String>,
    #[builder(setter(into))]
    #[doc = r"This represents IPN URLs which Pesapal will send notifications
    after the payment have been processed"]
    notification_id: String,
    #[builder(setter(into, strip_option), default)]
    #[doc = r"If your business has multiple stores / branches, you can define the name of the store / branch to which this particular payment will be accredited to."]
    branch: Option<String>,

    #[doc = r"The billing address of the customer"]
    billing_address: BillingAddress,
}

impl SubmitOrderBuilder<'_> {
    /// Validate that either the email address or the phone number is provided
    fn validate(&self) -> Result<(), String> {
        if let Some(billing_address) = &self.billing_address {
            if billing_address.email_address.is_none() && billing_address.phone_number.is_none() {
                return Err("either email or phone number must be provided.".to_string());
            }
        }

        Ok(())
    }
}

impl SubmitOrder<'_> {
    /// This initializes the SubmitOrder with the client and returns a builder
    pub(crate) fn builder(client: &PesaPal) -> SubmitOrderBuilder {
        SubmitOrderBuilder::default().client(client)
    }

    /// # Submit Order Request
    ///
    /// Sends the Order for payment processing
    ///
    /// ## Returns
    ///
    /// [SubmitOrderResponse] - Contains the necessary which conveys the
    /// successful result of the payment
    ///
    /// ## Errors
    ///
    /// [SubmitOrderError] - Incase the payment fails
    pub async fn send(self) -> PesaPalResult<SubmitOrderResponse> {
        let url = format!("{}/{SUBMIT_ORDER_REQUEST_URL}", self.client.env.base_url());

        let response = self
            .client
            .http_client
            .post(url)
            .bearer_auth(&self.client.authenticate().await?.token)
            .json::<SubmitOrderRequest>(&self.into())
            .send()
            .await?;

        let res: SubmitOrderResponse = response.json().await?;

        if let Some(error) = res.error {
            return Err(PesaPalError::SubmitOrderError(error));
        }

        Ok(res)
    }
}
