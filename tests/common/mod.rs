use pesapal::{Environment, PesaPal};
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

pub(crate) async fn pesapal_client() -> (PesaPal, MockServer) {
    dotenvy::dotenv().ok();
    let server = MockServer::start().await;

    let client = PesaPal::new(
        dotenvy::var("CONSUMER_KEY").expect("consumer_key not present"),
        dotenvy::var("CONSUMER_SECRET").expect("consumer_secret not present"),
        Environment::Custom(server.uri()),
    );

    let auth_response = json!({
        "token": "token",
        "expiry_date": "2021-08-26T12:29:30.5177702Z",
        "error": null,
        "status": "200",
        "message": "Request processed successfully"
    });

    Mock::given(method("POST"))
        .and(path("/api/Auth/RequestToken"))
        .respond_with(ResponseTemplate::new(200).set_body_json(auth_response))
        .expect(1)
        .mount(&server)
        .await;

    (client, server)
}
