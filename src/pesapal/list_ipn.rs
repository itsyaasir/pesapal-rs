//! List IPN URLs
//! This endpoint allows you to fetch all registered IPN URLs for a particular Pesapal merchant account.

use serde::Deserialize;
use serde_aux::prelude::{deserialize_default_from_null, deserialize_number_from_string};

use crate::PesaPalErrorResponse;

const LIST_IPN_URL: &str = "api/URLSetup/GetIpnList";

#[derive(Debug, Deserialize)]
pub struct IPNList {
    /// The notification URL Pesapal will send a status alert to
    pub url: String,
    /// Date and time the IPN URL was registered
    pub created_date: String,
    /// A unique identifier that's linked to he IPN endpoint URL
    pub ipn_id: String,
    /// Response code
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub status: u16,
    /// Response error if any
    #[serde(deserialize_with = "deserialize_default_from_null")]
    pub error: Option<PesaPalErrorResponse>,
}

/// Response from the list IPN endpoint
#[derive(Debug, Deserialize)]
pub struct IPNListResponse {
    pub ipns: Vec<IPNList>,
}

/// A builder for listing IPN URLs
#[derive(Debug, Clone)]
pub struct ListIPN<'pesa> {
    client: &'pesa crate::PesaPal,
}

impl<'pesa> ListIPN<'pesa> {
    /// Create a new instance of the ListIPN builder
    pub fn new(client: &'pesa crate::PesaPal) -> Self {
        Self { client }
    }

    /// # List IPN URLs
    ///
    ///  Send the request to PesaPal, and returns the IPN URLs
    /// registered for the merchant.
    ///
    /// # Returns
    ///
    /// Returns a list of IPN URLs registered for the merchant.
    pub async fn send(&self) -> crate::PesaPalResult<IPNListResponse> {
        let url = format!("{}/{}", self.client.env.base_url(), LIST_IPN_URL);
        let response = self
            .client
            .http_client
            .get(url)
            .bearer_auth(self.client.consumer_key.as_str())
            .send()
            .await?;
        let response = response.json::<IPNListResponse>().await?;

        response.ipns.iter().for_each(|ipn| {
            if let Some(error) = &ipn.error {
                eprintln!("Error: {:?}", error);
            }
        });

        Ok(response)
    }
}
