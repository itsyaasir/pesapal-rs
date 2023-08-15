//!## Pesapal-rs
//!
//! An unofficial Rust wrapper around the (PesaPal API)[https://developer.pesapal.com/] for accessing PesaPal
//! services.
//!
//!## Install
//! `Cargo.toml`
//!
//! ```toml
//! [dependencies]
//! pesapal = { git = "https://github.ciom/itsyaasir/pesapal-rs", branch = "main"}
//! ```
//!
//! In your lib or binary crate:
//! ```rs
//! use pesapal::PesaPal;
//! ```
//!
//!## Usage
//!
//!### Creating a `PesaPal` client
//! You will first need to create an instance of the `PesaPal` instance (the client). You are required to provide a **CONSUMER_KEY** and
//! **CONSUMER_SECRET**. [Here](https://developer.pesapal.com/api3-demo-keys.txt) is how you can get these credentials for the Pesapal sandbox
//! environment. It's worth noting that these credentials are only valid in the sandbox environment.
//!
//! These are the following ways you can instantiate `PesaPal`:
//!
//! ```rust,no_run
//! use pesapal::{PesaPal, Environment};
//! use std::env;
//! use dotenvy::dotenv;
//!
//! #[tokio::main]
//! async fn main() {
//!     dotenv().ok();
//!
//!     let client = PesaPal::new(
//!         env::var("CONSUMER_KEY").unwrap(),
//!         env::var("CONSUMER_SECRET").unwrap(),
//!         Environment::Sandbox,
//!     );
//! }
//! ```
//!
//! Since the `Environment` enum implements `FromStr` and `TryFrom` for `String` and `&str` types, you can call `Environment::from_str` or `Environment::try_from` to create an `Environment` type. This is ideal if the environment values are
//! stored in a `.env` or any other configuration file
//!
//! ```rust,no_run
//! use pesapal::{PesaPal, Environment};
//! use std::env;
//! use dotenvy::dotenv;
//! use std::str::FromStr;
//!
//! #[tokio::main]
//! async fn main() {
//!     dotenv().ok();
//!
//!     let client = PesaPal::new(
//!         env::var("CONSUMER_KEY").unwrap(),
//!         env::var("CONSUMER_SECRET").unwrap(),
//!         Environment::from_str("sandbox").unwrap()
//!     );
//! }
//! ```
//! If you intend to use in production, you will need to provide the
//! `CONSUMER_KEY` and `CONSUMER_SECRET` from the Pesapal
//!
//! ```rust,no_run
//! use pesapal::{PesaPal, Environment};
//! use std::env;
//! use dotenvy::dotenv;
//!
//! #[tokio::main]
//! async fn main() {
//!     dotenv().ok();
//!
//!     let client = PesaPal::new(
//!         env::var("CONSUMER_KEY").unwrap(),
//!         env::var("CONSUMER_SECRET").unwrap(),
//!         Environment::Sandbox
//!     );
//!
//!
//! }
//! ```
//!
//!### Services
//! The following services are currently available from the `PesaPal` client as
//! methods that return builders:
//!
//! * Submit Order - Sends the payment request that needs to be processed
//! ```rust,no_run
//! use pesapal::{PesaPal, Environment};
//! use std::env;
//! use dotenvy::dotenv;
//! use pesapal::pesapal::BillingAddress;
//!
//! #[tokio::main]
//! async fn main() {
//!     dotenv().ok();
//!
//!
//! let pesapal: PesaPal = PesaPal::new(
//!         env::var("CONSUMER_KEY").unwrap(),
//!         env::var("CONSUMER_SECRET").unwrap(),
//!         Environment::Sandbox
//! );
//!
//! let order = pesapal
//!     .submit_order()
//!     .currency("KES")
//!     .amount(2500)
//!     .description("Shopping")
//!     .branch("example")
//!     .callback_url("https://example.com")
//!     .cancellation_url("https://example.com")
//!     .notification_id("example")
//!     .billing_address(BillingAddress {
//!         email_address: Some("yasir@gmail.com".to_string()),
//!         ..Default::default()
//!      })
//!     .redirect_mode(RedirectMode::ParentWindow)
//!     .build()
//!     .unwrap();
//!
//! let response = order.send().await;
//!
//! assert!(response.is_ok())
//! }
//! ```
//!
//! * Refund - Sends refund request for a payment that was processed
//! ```rust,no_run
//! use pesapal::{PesaPal, Environment};
//! use std::env;
//! use dotenvy::dotenv;
//!
//! #[tokio::main]
//! async fn main() {
//!     dotenv().ok();
//!
//!
//! let pesapal: PesaPal = PesaPal::new(
//!         env::var("CONSUMER_KEY").unwrap(),
//!         env::var("CONSUMER_SECRET").unwrap(),
//!         Environment::Sandbox
//! );
//!
//! let refund_request = pesapal
//!     .refund()
//!     .amount(2500)
//!     .remarks("services not offered")
//!     .confirmation_code("AA22BB33CC")
//!     .username("John Doe")
//!     .build()
//!     .unwrap();
//!
//! let response = refund_request.send().await;
//!
//! assert!(response.is_ok())
//! }
//! ```
//!
//! More will be added progressively, pull requests welcome
//!
//!## Author
//!
//! **Yasir Shariff**
//!
//! * Twitter: [@itsyaasir](https://twitter.com/itsyaasir)
//! * Not affiliated with PesaPal.
//!
//!## License
//! This project is MIT licensed

pub mod environment;
pub mod error;
mod macros;
pub mod pesapal;

pub use environment::Environment;
pub use error::{PesaPalError, PesaPalErrorResponse, PesaPalResult};
pub use pesapal::refund::{Refund, RefundResponse};
pub use pesapal::{BillingAddress, PesaPal};
