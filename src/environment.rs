use std::str::FromStr;

use crate::env_from_string;
use crate::error::PesaPalError;

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub enum Environment {
    /// Production environment, used app which are in production
    Production,
    /// Sandbox environment which can be used for testing
    #[default]
    Sandbox,
    /// Custom Environment
    ///
    /// Mostly used for Mock testing Environment
    Custom(String),
}

impl Environment {
    /// Base URL for the two kinds of Environment
    #[must_use]
    pub fn base_url(&self) -> &str {
        match self {
            Self::Production => "https://pay.pesapal.com/v3",
            Self::Sandbox => "https://cybqa.pesapal.com/pesapalv3",
            Self::Custom(url) => url.as_str(),
        }
    }
}

impl TryFrom<&str> for Environment {
    type Error = PesaPalError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        env_from_string!(value.to_lowercase().as_str())
    }
}

impl TryFrom<String> for Environment {
    type Error = PesaPalError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        env_from_string!(value.to_lowercase().as_str())
    }
}

impl FromStr for Environment {
    type Err = PesaPalError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        env_from_string!(s.to_lowercase().as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_from_str() {
        let env = Environment::from_str("production").unwrap();
        assert_eq!(env, Environment::Production);
    }

    #[test]
    fn test_environment_from_string() {
        let env = Environment::try_from("production".to_string()).unwrap();
        assert_eq!(env, Environment::Production);
    }

    #[test]
    fn test_environment_with_different_case() {
        let envs = vec!["Production", "PRODUCTION", "production", "PrOdUcTiOn"];
        for env in envs {
            let env = Environment::try_from(env.to_string()).unwrap();
            assert_eq!(env, Environment::Production);
        }

        let envs = vec!["Sandbox", "SANDBOX", "sandbox", "SaNdBoX"];
        for env in envs {
            let env = Environment::try_from(env.to_string()).unwrap();
            assert_eq!(env, Environment::Sandbox);
        }
    }
}
