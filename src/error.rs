use crate::types::PesaPalErrorResponse;

#[derive(Debug, Clone, thiserror::Error)]
pub enum PesaPalError {
    #[error("Error occurred : {0}")]
    General(String),
    #[error("authentication error : {0}")]
    ApiError(PesaPalErrorResponse),

    #[error("reqwest error : {0}")]
    ReqwestError(String),
}

pub type PesaPalResult<T> = Result<T, PesaPalError>;

impl From<reqwest::Error> for PesaPalError {
    fn from(value: reqwest::Error) -> Self {
        panic!()
    }
}
