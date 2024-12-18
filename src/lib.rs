//! # GeoDB
//!
//! A collection of geographical information.
//!
#![deny(clippy::unwrap_used,clippy::expect_used,clippy::let_underscore_future)]

#[macro_use] extern crate serde;

pub mod timezone;
pub use timezone::*;