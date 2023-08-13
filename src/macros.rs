#[macro_export]
/// Simple macro for parsing `Environment` from string
macro_rules! env_from_string {
    ($env:expr) => {
        match $env {
            "production" => Ok(Environment::Production),
            "sandbox" => Ok(Environment::Sandbox),
            _ => Err(PesaPalError::UnsupportedEnvironment(
                "environment {value} not supported".to_string(),
            )),
        }
    };
}
