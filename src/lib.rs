// libsw: stopwatch library
// copyright (C) 2022  Ula Shipman <ula.hello@mailbox.org>
// licensed under MIT OR GPL-3.0-or-later

#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::pedantic)]

//! `libsw` is a comprehensive stopwatch implementation.
//!
//! See the [struct documentation](Stopwatch) for details.
//!
//! # Examples
//!
//! ```
//! use libsw::{Result, Stopwatch};
//!
//! use core::time::Duration;
//! use std::thread;
//!
//! fn main() -> Result<()> {
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

mod guard;
mod stopwatch;

pub use guard::Guard;
pub use stopwatch::{Error, Result, Stopwatch};

#[cfg(test)]
mod tests;
