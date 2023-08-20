mod common;

use pesapal::{BillingAddress, RedirectMode, SubmitOrderResponse};
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

use crate::common::pesapal_client;

#[tokio::test]
#[ignore = "test is currently failing"]
async fn test_submit_order_is_success() {
    let (client, server) = pesapal_client().await;
    let sample_response = SubmitOrderResponse {
        error: None,
        merchant_reference: "AA22BB33CC".to_string(),
        order_tracking_id: "AA22BB33CC".to_string(),
        redirect_url: "https://example.com".to_string(),
        status: 200.to_string(),
    };

    Mock::given(method("POST"))
        .and(path("api/Transactions/SubmitOrderRequest"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!(sample_response)))
        .expect(1)
        .mount(&server)
        .await;

    let order = client
        .submit_order()
        .currency("KES")
        .amount(2500)
        .description("Shopping")
        .callback_url("https://example.com")
        .cancellation_url("https://example.com")
        .notification_id("AABBCCDDEEFFGG")
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

    assert_eq!(sample_response, order)
}
