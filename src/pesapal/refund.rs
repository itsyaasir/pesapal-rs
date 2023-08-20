//! The refund request endpoint allow you to refund a charge that has
//! previously been processed but not yet refunded. Funds will be refunded to
//! the credit / debit card or mobile money wallet that was originally charged.
//!
//! The ability to process a refund has the following limitations:

//! - A refund has to be approved by the merchant.
//! - You can't refund more than what was originally collected.
//! - You can only refund payments with the status of COMPLETED.
//! - You can partially or fully refund a payment card payment.
//! - You can only fully refund a payment mobile payment.
//! - Refunds are performed in the currency of the original payment.
//! - Multiple refunds are not allowed. You can only request one refund against
//! a payment.

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::{PesaPal, PesaPalError, PesaPalResult};

const REFUND_REQUEST_URL: &str = "api/Transactions/RefundRequest";

#[derive(Debug, Clone, Serialize)]
pub struct RefundRequest {
    /// This refers to payment confirmation code that was returned by the
    /// processor
    pub confirmation_code: String,
    /// Amount to be refunded.
    pub amount: f64,
    /// Identity of the user who has initiated the refund.
    pub username: String,
    /// A brief description on the reason for the refund.
    pub remarks: String,
}

#[derive(Deserialize)]
pub struct RefundResponse {
    /// 200 - Refund received successfully and is being processed.
    /// 500 - Refund rejected.
    pub status: u16,
    /// A brief summary of the response received.
    pub message: String,
}

impl From<Refund<'_>> for RefundRequest {
    fn from(value: Refund<'_>) -> Self {
        Self {
            confirmation_code: value.confirmation_code,
            amount: value.amount,
            username: value.username,
            remarks: value.remarks,
        }
    }
}

#[derive(Builder, Debug)]
pub struct Refund<'pesa> {
    #[builder(pattern = "owned")]
    client: &'pesa PesaPal,
    #[builder(setter(into))]
    #[doc = "This refers to payment confirmation code that was returned by the payment processor"]
    confirmation_code: String,
    #[builder(setter(into))]
    #[doc = "Amount to be refunded."]
    amount: f64,
    #[builder(setter(into))]
    #[doc = "Identity of the user who has initiated the refund."]
    username: String,
    #[builder(setter(into))]
    #[doc = "A brief description on the reason for the refund."]
    remarks: String,
}

impl Refund<'_> {
    /// Initializes the builder for the Refund process
    pub(crate) fn builder(client: &PesaPal) -> RefundBuilder {
        RefundBuilder::default().client(client)
    }

    /// # Refund Request
    ///
    /// Sends the Refund Request
    ///
    /// ## Returns
    ///
    /// * status - 200 means your request to process the refund has been
    /// successfully received
    ///
    /// *NB* It doesn't mean the refund has been effected
    ///
    /// ## Errors
    ///
    /// [`PesaPalError::RefundError`] - with status 500 and error message incase the refund
    /// fails
    pub async fn send(self) -> PesaPalResult<RefundResponse> {
        let url = format!("{}/{REFUND_REQUEST_URL}", self.client.env.base_url());
        let response = self
            .client
            .http_client
            .post(url)
            .bearer_auth(&self.client.authenticate().await?)
            .json::<RefundRequest>(&self.into())
            .send()
            .await?;

        let res: RefundResponse = response.json().await?;

        if res.status == 500 {
            return Err(PesaPalError::RefundError(res.message));
        }

        Ok(res)
    }
}
