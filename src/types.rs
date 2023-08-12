use std::fmt::Display;

use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PesaPalErrorResponse {
    r#type: String,
    code: String,
    message: String,
}

impl Display for PesaPalErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationResponse {
    pub token: String,
    pub expiry_date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub status: String,
    pub message: String,
}
