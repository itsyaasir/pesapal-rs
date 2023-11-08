#[cfg(test)]
mod common;
#[cfg(test)]
mod it;

use httpmock::prelude::*;
use pesapal::{
    BillingAddress, PesaPalError, PesaPalErrorResponse, RedirectMode, SubmitOrderResponse,
};
use serde_json::json;

use crate::common::pesapal_client;

#[tokio::test]
async fn test_submit_order_is_success() {
    let (client, server) = pesapal_client().await;
    let sample_response = SubmitOrderResponse {
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

#[tokio::test]
async fn test_submit_order_can_send_request_without_cancellation_url() {
    let (client, server) = pesapal_client().await;
    let sample_response = SubmitOrderResponse {
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

#[tokio::test]
async fn test_submit_order_fails_when_inputs_are_missing() {
    let (client, server) = pesapal_client().await;
    let sample_response = SubmitOrderResponse {
        merchant_reference: "PQ54MC34FA129".to_string(),
        order_tracking_id: "awed-fxas-tr3a-zxqm-palu".to_string(),
        redirect_url: "https://example.com".to_string(),
        status: 200,
    };

    server
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
        .notification_id("OO5KAXIPA324AZ")
        .redirect_mode(RedirectMode::ParentWindow)
        .branch("EA")
        .billing_address(BillingAddress {
            email_address: Some("example@example.com".to_string()),
            ..Default::default()
        })
        .build();

    assert!(order.is_err());
    assert!(order
        .unwrap_err()
        .to_string()
        .contains("`callback_url` must be initialized"))
}

#[tokio::test]
async fn test_submit_order_returns_correct_error_response() {
    let (client, server) = pesapal_client().await;
    // We are going to mock unregistered callback url which usually returns error

    let err_response = json!({
        "code": "200",
        "error_type": "API Error",
        "message": "Unregistered IPN Url"
    });

    let mock = server
        .mock_async(|when, then| {
            when.path_contains("/api/Transactions/SubmitOrderRequest")
                .method(POST);
            then.json_body(json!(err_response)).status(500);
        })
        .await;

    let order = client
        .submit_order()
        .currency("KES")
        .amount(2500)
        .description("Shopping")
        .callback_url("https://example.com")
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
        .await;

    mock.assert_async().await;

    assert!(order.is_err());

    let expected_error_response: PesaPalErrorResponse =
        serde_json::from_value(err_response).expect("Failed to deserialize error response");

    if let Err(actual_error) = order {
        match actual_error {
            PesaPalError::SubmitOrderError(actual_error_response) => {
                assert_eq!(actual_error_response, expected_error_response);
            }
            _ => {
                panic!("Expected SubmitOrderError but got a different error type");
            }
        }
    } else {
        panic!("Expected an error but got a successful result");
    }
}
