// libsw: stopwatch library
// copyright (C) 2022-2023 Ula Shipman <ula.hello@mailbox.org>
// licensed under MIT OR Apache-2.0

//! `libsw` is a comprehensive stopwatch implementation.
//!
//! It offers [checked stopping](StopwatchImpl::checked_stop) and
//! [arithmetic](StopwatchImpl::checked_add), [precise
//! control](StopwatchImpl::start_at) over when operations occur, and supports
//! [arbitrary timekeeping types](Instant).
//!
//! If you want to do benchmarking, please use something like
//! [Criterion](https://docs.rs/criterion).
//!
//! # Introduction
//!
//! `libsw` provides the [`StopwatchImpl`] type as a stopwatch.
//!
//! This implementation is agnostic to the timekeeping type used, by virtue of
//! being generic. Any type `I` that implements the [`Instant`] trait (as in
//! `StopwatchImpl<I>`) can be used for timekeeping.
//!
//! `Instant` is implemented for several timekeeping types out of the box (see
//! [timekeeping support](#timekeeping-support)). If present, these
//! implementations are exposed as type aliases.
//!
//! # Features
//!
//! | Name             | Features enabled                | Description                                                                                             |
//! |------------------|---------------------------------|---------------------------------------------------------------------------------------------------------|
//! | `default`        | `std_instant`, `std_systemtime` | Enabled by default.                                                                                     |
//! | `std`            |                                 | Depends on the standard library. Implements `std::error::Error` for [`Error`].                          |
//! | `nightly`        |                                 | Implements `core::error::Error` for [`Error`] **if** `std` is not enabled. Requires a nightly compiler. |
//! | `std_instant`    | `std`                           | Implements [`Instant`] for `std::time::Instant`. Exposes `Sw` type alias.                               |
//! | `std_systemtime` | `std`                           | Implements [`Instant`] for `std::time::SystemTime`. Exposes `SystemSw` type alias.                      |
//! | `tokio`          | `std`                           | Implements [`Instant`] for `tokio::time::Instant`. Exposes `TokioSw` type alias.                        |
//! | `coarsetime`     | `std`                           | Implements [`Instant`] for `coarsetime::Instant`. Exposes `CoarseSw` type alias.                        |
//! | `quanta`         | `std`                           | Implements [`Instant`] for `quanta::Instant`. Exposes `QuantaSw` type alias.                            |
//! | `time`           | `std`                           | Deprecated. Implements [`Instant`] for `time::Instant`. Exposes `TimeSw` type alias.                    |
//!
//! ## Timekeeping support
//!
//! `libsw` can be used with any timekeeping type that implements [`Instant`],
//! as long as the appropriate feature flag is enabled.
//!
//! See `Instant`'s [documentation](Instant#provided-implementations) for a list
//! of types supported out of the box.
//!
//! ## `no_std` support
//!
//! The `std` feature flag unsets `#[no_std]`. It is enabled by default, but you
//! can disable it by disabling the default features.
//!
//! In `Cargo.toml`,
//!
//! ```toml
//! [dependencies]
//! # replace '...' with the appropriate version
//! libsw = { version = ..., default-features = false }
//! ```
//!
//! # Compiler support
//!
//! Standalone, the minimum supported version of Rust is `1.61.0`.
//! Adding dependencies may bump this.
//!
//! # Safety
//!
//! `libsw` contains no unsafe code (`#![forbid(unsafe_code)]`).

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(all(feature = "nightly", not(feature = "std")), feature(error_in_core))]
#![cfg_attr(doc_cfg, feature(doc_cfg))]
#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::pedantic, clippy::cargo)]

extern crate core;

mod error;
mod guard;
mod stopwatch;

pub use crate::error::{Error, Result};
pub use crate::guard::Guard;
pub use crate::stopwatch::StopwatchImpl;
pub use ::libsw_core::Instant;

/// Alias to [`StopwatchImpl`] using the standard library's
/// [`Instant`](std::time::Instant) type.
///
/// This is the "default" stopwatch.
#[cfg(feature = "std_instant")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "std_instant")))]
pub type Sw = StopwatchImpl<::std::time::Instant>;

/// Deprecated alias to the "default" stopwatch.
#[cfg(feature = "std_instant")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "std_instant")))]
#[deprecated(
    since = "3.0.0",
    note = "use `Sw` instead, an alias to `StopwatchImpl<std::time::Instant>`"
)]
pub type Stopwatch = Sw;

/// Alias to [`StopwatchImpl`] using the standard library's
/// [`SystemTime`](std::time::SystemTime) type.
#[cfg(feature = "std_systemtime")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "std_systemtime")))]
pub type SystemSw = StopwatchImpl<::std::time::SystemTime>;

/// Alias to [`StopwatchImpl`] using Tokio's [`Instant`](tokio::time::Instant)
/// type.
#[cfg(feature = "tokio")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "tokio")))]
pub type TokioSw = StopwatchImpl<::tokio::time::Instant>;

/// Alias to [`StopwatchImpl`] using the `coarsetime` crate's
/// [`Instant`](coarsetime::Instant) type.
#[cfg(feature = "coarsetime")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "coarsetime")))]
pub type CoarseSw = StopwatchImpl<::coarsetime::Instant>;

/// Alias to [`StopwatchImpl`] using the `quanta` crate's
/// [`Instant`](quanta::Instant) type.
#[cfg(feature = "quanta")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "quanta")))]
pub type QuantaSw = StopwatchImpl<::quanta::Instant>;

/// Alias to [`StopwatchImpl`] using the `time` crate's
/// [`Instant`](time::Instant) type.
#[allow(deprecated)]
#[cfg(feature = "time")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "time")))]
#[deprecated(
    note = "the `time` crate has deprecated `time::Instant` in favor of the `time::ext::InstantExt` trait used with `std::time::Instant`"
)]
pub type TimeSw = StopwatchImpl<::time::Instant>;

#[cfg(test)]
mod tests;
