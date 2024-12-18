//! # GeoDB
//!
//! A collection of geographical information.
//!
#![deny(clippy::unwrap_used,clippy::expect_used,clippy::let_underscore_future)]

#[macro_use] extern crate serde;

pub mod currency;
pub use currency::*;

pub mod timezone;
mod country;

pub use timezone::*;