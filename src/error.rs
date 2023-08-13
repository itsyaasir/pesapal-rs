use serde::Deserialize;

#[derive(Debug, Clone, thiserror::Error)]
pub enum PesaPalError {
    #[error("internal error occurred : {0}")]
    Internal(String),
    #[error("authentication error : {0}")]
    AuthenticationError(PesaPalErrorResponse),

    #[error("submit order failed :{0}")]
    SubmitOrderError(PesaPalErrorResponse),
    #[error("reqwest error : {0}")]
    ReqwestError(String),
    #[error("unsupported environment {0}")]
    UnsupportedEnvironment(String),
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]

pub struct PesaPalErrorResponse {
    pub code: String,
    #[serde(rename = "error_type")]
    pub error_type: String,
    pub message: String,
}

impl std::fmt::Display for PesaPalErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "error type: {} code: {} message: {}",
            self.error_type, self.code, self.message
        )
    }
}

pub type PesaPalResult<T> = Result<T, PesaPalError>;

impl From<reqwest::Error> for PesaPalError {
    fn from(value: reqwest::Error) -> Self {
        Self::ReqwestError(value.to_string())
    }
}

impl From<serde_json::Error> for PesaPalError {
    fn from(value: serde_json::Error) -> Self {
        Self::Internal(value.to_string())
    }
}