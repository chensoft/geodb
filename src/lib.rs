#![doc = include_str!("../README.md")]
#![allow(non_camel_case_types,clippy::needless_doctest_main)]
#![deny(clippy::unwrap_used,clippy::expect_used,clippy::let_underscore_future)]

#[macro_use] extern crate serde;
#[macro_use] extern crate strum;

include!("../out/continent.rs");
include!("../out/country.rs");
include!("../out/currency.rs");
include!("../out/language.rs");
include!("../out/subdivision.rs");