#[cfg(test)]
mod common;

use httpmock::prelude::*;
use pesapal::{BillingAddress, RedirectMode, SubmitOrderResponse};
use serde_json::json;

use crate::common::pesapal_client;

#[tokio::test]
async fn test_submit_order_is_success() {
    let (client, server) = pesapal_client().await;
    let sample_response = SubmitOrderResponse {
        error: None,
        merchant_reference: "PQ54MC34FA129".to_string(),
        order_tracking_id: "awed-fxas-tr3a-zxqm-palu".to_string(),
        redirect_url: "https://example.com".to_string(),
        status: 200,
    };

    let mock = server
        .mock_async(|when, then| {
            when.path_contains("/api/Transactions/SubmitOrderRequest")
                .method(POST);
            then.json_body(json!(sample_response)).status(200);
        })
        .await;

    let order = client
        .submit_order()
        .currency("KES")
        .amount(2500)
        .description("Shopping")
        .callback_url("https://example.com")
        .cancellation_url("https://example.com")
        .notification_id("OO5KAXIPA324AZ")
        .redirect_mode(RedirectMode::ParentWindow)
        .branch("EA")
        .billing_address(BillingAddress {
            email_address: Some("example@example.com".to_string()),
            ..Default::default()
        })
        .build()
        .unwrap()
        .send()
        .await
        .unwrap();

    mock.assert_async().await;

    assert_eq!(sample_response, order)
}
