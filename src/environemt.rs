use error::PesaPalError;
use reqwest::Client as HttpClient;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum Environment {
    Production,
    Sandbox,
}

impl TryFrom<&str> for Environment {
    type Error = PesaPalError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "production" => Ok(Self::Production),
            "sandbox" => Ok(Self::Sandbox),
            _ => Err(PesaPalError::General(
                "enivonment {value} not supported".to_string(),
            )),
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = PesaPalError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "production" => Ok(Self::Production),
            "sandbox" => Ok(Self::Sandbox),
            _ => Err(PesaPalError::General(
                "enivonment {value} not supported".to_string(),
            )),
        }
    }
}

impl FromStr for Environment {
    type Err = PesaPalError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "production" => Ok(Self::Production),
            "sandbox" => Ok(Self::Sandbox),
            _ => Err(PesaPalError::General(
                "enivonment {value} not supported".to_string(),
            )),
        }
    }
}

// TODO:: USe macros for the conversion- duplicated code
pub trait ApiEnvironment {
    fn base_url(&self) -> &str;
}

impl ApiEnvironment for Environment {
    fn base_url(&self) -> &str {
        match self {
            Environment::Production => "https://pay.pesapal.com/v3",
            Environment::Sandbox => "https://cybqa.pesapal.com/pesapalv3",
        }
    }
}
