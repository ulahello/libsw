// libsw: stopwatch library
// copyright (C) 2022  Ula Shipman <ula.hello@mailbox.org>
// licensed under MIT OR GPL-3.0-or-later

use core::fmt;
use std::error;

/// Alias for `Result<T, Error>`.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors associated with a [`Stopwatch`](crate::Stopwatch).
///
/// # Examples
///
/// ```
/// # use libsw::{Error, Result, Stopwatch};
/// # fn main() -> Result<()> {
/// let mut sw = Stopwatch::new_started();
/// assert_eq!(sw.start(), Err(Error::AlreadyStarted));
/// sw.stop()?;
/// assert_eq!(sw.stop(), Err(Error::AlreadyStopped));
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Error {
    /// Tried to start the stopwatch while it was already running
    AlreadyStarted,
    /// Tried to stop the stopwatch when it was already stopped
    AlreadyStopped,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), fmt::Error> {
        f.write_str(match self {
            Self::AlreadyStarted => "stopwatch already started",
            Self::AlreadyStopped => "stopwatch already stopped",
        })
    }
}

impl error::Error for Error {}
