use httpmock::prelude::*;
use pesapal::{Environment, PesaPal};
use serde_json::json;

pub(crate) async fn pesapal_client() -> (PesaPal, MockServer) {
    dotenvy::dotenv().ok();

    let server = MockServer::start_async().await;

    let client = PesaPal::new(
        dotenvy::var("CONSUMER_KEY").expect("consumer_key not present"),
        dotenvy::var("CONSUMER_SECRET").expect("consumer_secret not present"),
        Environment::Custom(server.base_url()),
    );

    let auth_response = json!({
        "token": "token",
        "expiryDate": "2021-08-26T12:29:30.5177702Z",
        "error": null,
        "status": "200",
        "message": "Request processed successfully"
    });

    server
        .mock_async(|when, then| {
            when.path_contains("/api/Auth/RequestToken").method(POST);
            then.json_body(auth_response).status(200);
        })
        .await;

    (client, server)
}
