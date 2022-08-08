// libsw: stopwatch library
// copyright (C) 2022  Ula Shipman <ula.hello@mailbox.org>
// licensed under MIT OR GPL-3.0-or-later

#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::pedantic)]

//! Defines an abstraction for stopwatches
//!
//! This is a straightforward stopwatch implementation. It doesn't implement
//! laps.
//!
//! None of the methods in [`Stopwatch`] panic or call functions which are
//! documented to panic.

mod stopwatch;
pub use stopwatch::*;

#[cfg(test)]
mod tests;
