use dotenvy::dotenv;
use pesapal_rs::environment::Environment;
use pesapal_rs::pesapal::{BillingAddress, PesaPal};

#[tokio::test]
async fn test_submit_order() {
    dotenv().ok();

    let client = PesaPal::new(
        dotenvy::var("CONSUMER_KEY").unwrap(),
        dotenvy::var("CONSUMER_SECRET").unwrap(),
        Environment::Sandbox,
    );

    let response = client
        .submit_order()
        .currency("KES")
        .amount(2500)
        .description("Shopping")
        .callback_url("https://example.com")
        .cancellation_url("https://example.com")
        .notification_id("example")
        .billing_address(BillingAddress {
            email_address: Some("yasir@gmail.com".to_string()),
            ..Default::default()
        })
        .build()
        .unwrap();

    let res = response.send().await;

    assert!(res.is_ok())
}
