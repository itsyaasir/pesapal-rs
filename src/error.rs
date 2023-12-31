use serde::Deserialize;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum PesaPalError {
    #[error("internal error occurred : {0}")]
    Internal(String),
    #[error("authentication error : {0}")]
    AuthenticationError(PesaPalErrorResponse),

    #[error("submit order failed :{0}")]
    SubmitOrderError(PesaPalErrorResponse),

    #[error("refund request failed {0}")]
    RefundError(String),
    #[error("register IPN URL error")]
    RegisterIPNError(PesaPalErrorResponse),
    #[error("transaction status error : {0:?}")]
    TransactionStatusError(TransactionStatusError),
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error("unsupported environment {0}")]
    UnsupportedEnvironment(String),
    #[error("validation error")]
    ValidationError(String),
}

/// Error response for the Pesapal API error
#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct PesaPalErrorResponse {
    pub code: String,
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
/// Type alias for the result
pub type PesaPalResult<T> = Result<T, PesaPalError>;

impl From<serde_json::Error> for PesaPalError {
    fn from(value: serde_json::Error) -> Self {
        Self::Internal(value.to_string())
    }
}

/// Error response for the `TransactionStatus` Endpoint
#[derive(Debug, Deserialize)]
#[non_exhaustive]
pub struct TransactionStatusError {
    pub error_type: String,
    pub code: String,
    pub message: String,
    pub call_back_url: String,
}
