// libsw: stopwatch library
// copyright (C) 2022 Ula Shipman <ula.hello@mailbox.org>
// licensed under MIT OR Apache-2.0

#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::pedantic, clippy::nursery, clippy::cargo)]

//! `libsw` is a comprehensive stopwatch implementation.
//!
//! The minimum supported version of Rust is `1.58.1`.
//!
//! # Examples
//!
//! ```
//! use libsw::Stopwatch;
//!
//! use core::time::Duration;
//! use std::thread;
//!
//! fn main() -> libsw::Result<()> {
//!     let mut sw = Stopwatch::new();
//!
//!     sw.start()?;
//!     thread::sleep(Duration::from_millis(100));
//!     sw.stop()?;
//!
//!     println!("thread slept for {:?}", sw.elapsed());
//!
//!     Ok(())
//! }
//! ```
//!
//! # Safety
//!
//! `libsw` contains no unsafe code (`#![forbid(unsafe_code)]`).

mod error;
mod guard;
mod stopwatch;

pub use error::{Error, Result};
pub use guard::Guard;
pub use stopwatch::Stopwatch;

#[cfg(test)]
mod tests;
