#![deny(
    unstable_features,
    unused_must_use,
    unused_mut,
    unused_imports,
    unused_import_braces
)]
#![allow(clippy::needless_doctest_main)]
#![allow(clippy::needless_doctest_main)]
#[macro_use]
extern crate error_chain;
pub mod errors;
pub mod api;
pub mod utils;
pub mod market;
pub mod client;
pub mod model;
pub mod config;
