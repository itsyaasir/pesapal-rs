//!## Pesapal-rs
//!
//! An unofficial Rust wrapper around the (`PesaPal` API)[<https://developer.pesapal.com/>] for accessing `PesaPal`
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
//! You will first need to create an instance of the `PesaPal` instance (the client). You are required to provide a **`CONSUMER_KEY`** and
//! **`CONSUMER_SECRET`**. [Here](https://developer.pesapal.com/api3-demo-keys.txt) is how you can get these credentials for the Pesapal sandbox
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
//! use pesapal::pesapal::submit_order::RedirectMode;
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
//! * Register IPN URL - Register IPN URL
//! ```rust,no_run
//! use pesapal::{PesaPal, Environment};
//! use std::env;
//! use dotenvy::dotenv;
//! use pesapal::NotificationType;
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
//! let register_ipn_response = pesapal
//!     .register_ipn_url()
//!     .url("example")
//!     .ipn_notification_type(NotificationType::Get)
//!     .build()
//!     .unwrap();
//!
//! let response = register_ipn_response.send().await;
//!
//! assert!(response.is_ok())
//! }
//! ```
//!
//! * List IPN URL - List IPN URL
//! ```rust,no_run
//! use pesapal::{PesaPal, Environment};
//! use std::env;
//! use dotenvy::dotenv;
//! use pesapal::pesapal::list_ipn::IPNListResponse;
//!
//! #[tokio::main]
//! async fn main() {
//!    dotenv().ok();
//!
//!   let pesapal: PesaPal = PesaPal::new(
//!        env::var("CONSUMER_KEY").unwrap(),
//!        env::var("CONSUMER_SECRET").unwrap(),
//!        Environment::Sandbox
//!  );
//!
//! let response: IPNListResponse = pesapal.list_ipn_urls().send().await.unwrap();
//! }
//! ```
//!
//! * Transaction Status - Transaction Status
//! ```rust,no_run
//! use pesapal::{PesaPal, Environment};
//! use std::env;
//! use dotenvy::dotenv;
//! use pesapal::pesapal::transaction_status::TransactionStatusResponse;
//!
//! #[tokio::main]
//! async fn main() {
//!    dotenv().ok();
//!
//!  let pesapal: PesaPal = PesaPal::new(
//!       env::var("CONSUMER_KEY").unwrap(),
//!      env::var("CONSUMER_SECRET").unwrap(),
//!     Environment::Sandbox
//! );
//!
//! let response: TransactionStatusResponse = pesapal
//!    .transaction_status()
//!     .order_tracking_id("example")
//!     .build()
//!     .unwrap()
//!     .send()
//!     .await
//!     .unwrap();
//!
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
//! * Not affiliated with `PesaPal`.
//!
//!## License
//! This project is MIT licensed

#[deny(warnings)]
mod environment;
mod error;
mod macros;
mod pesapal;

pub use environment::Environment;
pub use error::{PesaPalError, PesaPalErrorResponse, PesaPalResult, TransactionStatusError};

pub use crate::pesapal::list_ipn::{IPNList, IPNListResponse, ListIPN};
pub use crate::pesapal::refund::{Refund, RefundRequest, RefundResponse};
pub use crate::pesapal::register_ipn::{NotificationType, RegisterIPN, RegisterIPNResponse};
pub use crate::pesapal::submit_order::{
    BillingAddress, RedirectMode, SubmitOrder, SubmitOrderRequest, SubmitOrderResponse,
};
pub use crate::pesapal::transaction_status::{
    StatusCode, TransactionStatus, TransactionStatusBuilder, TransactionStatusResponse,
};
pub use crate::pesapal::PesaPal;
