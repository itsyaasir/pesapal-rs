use cached::proc_macro::cached;
use cached::TimedSizedCache;
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Deserializer};
use serde_aux::prelude::{deserialize_default_from_null, deserialize_number_from_string};
use serde_json::json;

use crate::{PesaPal, PesaPalError, PesaPalErrorResponse};

/// Response returned from the authentication function
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationResponse {
    /// Access token which is used as the Bearer-Auth-Token
    pub token: String,
    #[serde(deserialize_with = "deserialize_utc_from_string")]
    /// Expiry date of the token
    pub expiry_date: DateTime<Utc>,
    /// Error response if present
    #[serde(deserialize_with = "deserialize_default_from_null")]
    pub error: Option<PesaPalErrorResponse>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub status: u16,
    pub message: String,
}

impl std::fmt::Display for AuthenticationResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "token :{} expiration_date: {}",
            self.token, self.expiry_date
        )
    }
}

/// Manual implementation of deserializing `UTC` from string
fn deserialize_utc_from_string<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;

    Ok(NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S%.fZ")
        .map_err(serde::de::Error::custom)?
        .and_utc())
}

/// Access token which is cached
pub type AccessToken = String;

#[cached(
    name = "AUTH_CACHE",
    type = "TimedSizedCache<String,AccessToken>",
    create = "{ TimedSizedCache::with_size_and_lifespan_and_refresh(1, 300,    true) }",
    convert = r#"{ format!("{}", client.consumer_key) }"#,
    result = true
)]
pub async fn auth(client: &PesaPal) -> Result<AccessToken, PesaPalError> {
    let url = format!("{}/api/Auth/RequestToken", client.env.base_url());
    let payload = json!({
        "consumer_key": client.consumer_key,
        "consumer_secret": client.consumer_secret
    });

    let response = client.http_client.post(url).json(&payload).send().await?;

    if response.status().is_success() {
        let value: AuthenticationResponse = response.json::<_>().await?;
        return Ok(value.token);
    }

    let err = response.json().await?;
    Err(PesaPalError::AuthenticationError(err))
}

#[cfg(test)]
mod tests {

    use cached::Cached;
    use dotenvy::dotenv;

    use super::*;
    use crate::Environment;

    #[test]
    fn test_deserialize_utc_from_string() {
        let input = "2021-08-26T12:29:30.5177702Z";

        let expected_datetime: DateTime<Utc> =
            NaiveDateTime::parse_from_str(input, "%Y-%m-%dT%H:%M:%S%.fZ")
                .unwrap()
                .and_utc();

        let json_str = format!(
            r#"
            {{
                "expiryDate": "{input}",
                "token":"1",
                "status":"200",
                "message": "success",
                "error": null
            }}
        "#
        );

        let response: AuthenticationResponse = serde_json::from_str(&json_str).unwrap();
        assert_eq!(response.expiry_date, expected_datetime);
    }

    #[tokio::test]
    async fn test_cached_access_token() {
        dotenv().ok();

        let client = PesaPal::new(
            dotenvy::var("CONSUMER_KEY").unwrap(),
            dotenvy::var("CONSUMER_SECRET").unwrap(),
            Environment::Sandbox,
        );
        auth_prime_cache(&client).await.unwrap();

        let mut cache = AUTH_CACHE.lock().await;

        assert!(cache.cache_get(&client.consumer_key).is_some());
        assert_eq!(cache.cache_hits().unwrap(), 1);
        assert_eq!(cache.cache_capacity().unwrap(), 1);
    }
}
