// libsw: stopwatch library
// copyright (C) 2022  Ula Shipman <ula.hello@mailbox.org>
// licensed under MIT OR GPL-3.0-or-later

#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::pedantic)]

//! This is a straightforward stopwatch implementation. It doesn't implement
//! laps.
//!
//! See the [struct documentation](Stopwatch) for details.
//!
//! # Safety
//!
//! `libsw` contains no unsafe code (`#![forbid(unsafe_code)]`).

mod stopwatch;
pub use stopwatch::*;

#[cfg(test)]
mod tests;
