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
//! # Example
//!
//! The following (contrived) example shows the basic features of the crate. You
//! are encouraged to read the examples provided for methods of
//! [`StopwatchImpl`] and [`Guard`] for more complex use cases.
//!
//! If you want to do benchmarking, please use something like
//! [Criterion](https://docs.rs/criterion).
//!
//! ```
//! use libsw::{Guard, Sw};
//!
//! use core::time::Duration;
//! use std::thread;
//! use std::time::Instant;
//!
//! fn main() {
//!     if let Err(err) = try_main() {
//!         // libsw::Error implements display, this
//!         // will explain the error
//!         eprintln!("error: {err}");
//!     }
//! }
//!
//! fn try_main() -> libsw::Result<()> {
//!     let mut sw = Sw::new();
//!
//!     // time how long `expensive` takes
//!     sw.start()?;
//!     let x = expensive();
//!     sw.stop()?;
//!
//!     println!(
//!         "expensive function returned {x} after {:?}",
//!         sw.elapsed()
//!     );
//!
//!     sw.reset();
//!
//!     // another way to do this is with guards. the
//!     // guard will keep `sw` running until it's
//!     // dropped.
//!     let y = expensive_timed(sw.guard()?);
//!     println!(
//!         "same function returned {y} after {:?}",
//!         sw.elapsed()
//!     );
//!
//!     // uh-oh, this will fail! the stopwatch is already stopped.
//!     sw.stop()?;
//!
//!     Ok(())
//! }
//!
//! fn expensive() -> u32 {
//!     thread::sleep(Duration::from_millis(100));
//!     1
//! }
//!
//! fn expensive_timed<I: libsw::Instant>(_guard: Guard<'_, I>) -> u32 {
//!     // guard is dropped when the function returns,
//!     // automatically stopping the guarded
//!     // stopwatch
//!     expensive()
//! }
//! ```
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
//! | `time`           | `std`                           | Implements [`Instant`] for `time::Instant`. Exposes `TimeSw` type alias. Bumps MSRV to `1.62.1`.        |
//! | `coarsetime`     | `std`                           | Implements [`Instant`] for `coarsetime::Instant`. Exposes `CoarseSw` type alias.                        |
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
//! ## Compiler support
//!
//! The minimum supported version of Rust is `1.61.0`.
//!
//! ## Safety
//!
//! `libsw` contains no unsafe code (`#![forbid(unsafe_code)]`).

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(all(feature = "nightly", not(feature = "std")), feature(error_in_core))]
#![cfg_attr(doc_cfg, feature(doc_cfg))]
#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::pedantic, clippy::nursery, clippy::cargo)]

mod error;
mod guard;
mod instant;
mod instant_impls;
mod stopwatch;

pub use error::{Error, Result};
pub use guard::Guard;
pub use instant::Instant;
pub use stopwatch::StopwatchImpl;

/// Alias to [`StopwatchImpl`] using the standard library's
/// [`Instant`](std::time::Instant) type.
///
/// This is the "default" stopwatch.
#[cfg(feature = "std_instant")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "std_instant")))]
pub type Sw = StopwatchImpl<std::time::Instant>;

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
pub type SystemSw = StopwatchImpl<std::time::SystemTime>;

/// Alias to [`StopwatchImpl`] using Tokio's [`Instant`](tokio::time::Instant)
/// type.
#[cfg(feature = "tokio")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "tokio")))]
pub type TokioSw = StopwatchImpl<tokio::time::Instant>;

/// Alias to [`StopwatchImpl`] using the `time` crate's
/// [`Instant`](time::Instant) type.
#[cfg(feature = "time")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "time")))]
pub type TimeSw = StopwatchImpl<time::Instant>;

/// Alias to [`StopwatchImpl`] using the `coarsetime` crate's
/// [`Instant`](coarsetime::Instant) type.
#[cfg(feature = "coarsetime")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "coarsetime")))]
pub type CoarseSw = StopwatchImpl<coarsetime::Instant>;

#[cfg(test)]
mod tests;
