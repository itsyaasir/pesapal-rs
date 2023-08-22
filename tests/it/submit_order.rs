use dotenvy::dotenv;
use pesapal::{BillingAddress, Environment, NotificationType, PesaPal, RedirectMode};

#[tokio::test]
async fn test_submit_order() {
    dotenv().ok();
    let client = PesaPal::new(
        dotenvy::var("CONSUMER_KEY").unwrap(),
        dotenvy::var("CONSUMER_SECRET").unwrap(),
        Environment::Sandbox,
    );

    // let response = client
    //     .register_ipn_url()
    //     .url("http://example.com")
    //     .ipn_notification_type(NotificationType::Get)
    //     .build()
    //     .unwrap()
    //     .send()
    //     .await
    //     .unwrap();

    client
        .submit_order()
        .currency("KES")
        .amount(2500)
        .description("Shopping")
        .callback_url("https://example.com")
        .cancellation_url("https://example.com")
        .notification_id("response.ipn_id")
        .redirect_mode(RedirectMode::ParentWindow)
        .branch("Branch")
        .billing_address(BillingAddress {
            email_address: Some("yasir@gmail.com".to_string()),
            ..Default::default()
        })
        .build()
        .unwrap()
        .send()
        .await
        .unwrap();
}
