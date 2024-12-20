//! # GeoDB
//!
//! A collection of geographical information.
//!
#![deny(clippy::unwrap_used,clippy::expect_used,clippy::let_underscore_future)]

#[macro_use] extern crate serde;
#[macro_use] extern crate strum;

pub mod continent;
pub mod country;
pub mod currency;
pub mod timezone;

pub use continent::*;
pub use country::*;
pub use currency::*;
// pub use timezone::*;