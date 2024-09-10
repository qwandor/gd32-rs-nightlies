//! Peripheral access API for GD32C1 microcontrollers
//! (generated using [svd2rust](https://github.com/rust-embedded/svd2rust)
//! 0.32.0)
//!
//! You can find an overview of the API here:
//! [svd2rust/#peripheral-api](https://docs.rs/svd2rust/0.32.0/svd2rust/#peripheral-api)
//!
//! For more details see the README here:
//! [gd32-rs](https://github.com/gd32-rust/gd32-rs)
//!
//! This crate supports all GD32C1 devices; for the complete list please
//! see:
//! [gd32c1](https://github.com/gd32-rust/gd32-rs-nightlies/tree/main/gd32c1)
//!
//! Due to doc build limitations, not all devices may be shown on docs.rs;
//! a representative few have been selected instead. For a complete list of
//! available registers and fields see: [gd32-rs Device Coverage](https://gd32-rust.github.io/gd32-rs/)

#![allow(non_camel_case_types)]
#![no_std]

mod generic;
pub use self::generic::*;

#[cfg(feature = "gd32c103")]
pub mod gd32c103;

#[cfg(feature = "gd32c113")]
pub mod gd32c113;

