// libsw: stopwatch library
// copyright (C) 2022-2023 Ula Shipman <ula.hello@mailbox.org>
// licensed under MIT OR Apache-2.0

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::pedantic, clippy::nursery, clippy::cargo)]

//! `libsw` is a comprehensive stopwatch implementation.
//!
//! # Examples
//!
//! The following (contrived) example shows the basic features of the crate. You
//! are encouraged to read the examples provided for methods of [`Stopwatch`]
//! and [`Guard`] for more complex use cases.
//!
//! If you want to do benchmarking, please use something like
//! [Criterion](https://docs.rs/criterion).
//!
//! ```
//! use libsw::{Guard, Stopwatch};
//!
//! use core::time::Duration;
//! use std::thread;
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
//!     let mut sw = Stopwatch::new();
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
//!     sw.stop()?; // uh-oh, this will fail! the
//!                 // stopwatch is already stopped.
//!
//!     Ok(())
//! }
//!
//! fn expensive() -> u32 {
//!     thread::sleep(Duration::from_millis(100));
//!     1
//! }
//!
//! fn expensive_timed(_guard: Guard<'_>) -> u32 {
//!     // guard is dropped when the function returns,
//!     // automatically stopping the guarded
//!     // stopwatch
//!     expensive()
//! }
//! ```
//!
//! # Timekeeping support
//!
//! `libsw` can be used with any timekeeping type that implements [`Instant`].
//!
//! See `Instant`'s [documentation](Instant#provided-implementations) for a list of types supported out of the
//! box.
//!
//! # `no_std` support
//!
//! The `std` feature flag determines whether `#[no_std]` is set. It is enabled
//! by default, but you can disable it by disabling the default features.
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
//! The minimum supported version of Rust is `1.58.1`.
//!
//! # Safety
//!
//! `libsw` contains no unsafe code (`#![forbid(unsafe_code)]`).

mod error;
mod guard;
mod instant;
mod instant_impls;
mod stopwatch;

pub use error::{Error, Result};
pub use guard::Guard;
pub use instant::Instant;
pub use stopwatch::Stopwatch;

#[cfg(test)]
mod tests;
