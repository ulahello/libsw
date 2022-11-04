// libsw: stopwatch library
// copyright (C) 2022 Ula Shipman <ula.hello@mailbox.org>
// licensed under MIT OR Apache-2.0

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
/// assert_eq!(sw.start(), Err(Error::SwStart));
/// sw.stop()?;
/// assert_eq!(sw.stop(), Err(Error::SwStop));
/// # Ok(())
/// # }
/// ```
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Error {
    /// Returned by [`Stopwatch::start`](crate::Stopwatch::start) and
    /// [`Stopwatch::start_at`](crate::Stopwatch::start_at). Expects a stopped
    /// stopwatch.
    SwStart,

    /// Returned by [`Stopwatch::stop`](crate::Stopwatch::stop) and
    /// [`Stopwatch::stop_at`](crate::Stopwatch::stop_at). Expects a running
    /// stopwatch.
    SwStop,

    /// Returned by [`Stopwatch::guard`](crate::Stopwatch::guard) or
    /// [`Stopwatch::guard_at`](crate::Stopwatch::guard_at). Expects a stopped
    /// stopwatch.
    SwGuard,

    /// Returned by [`Guard::new`](crate::Guard::new). Expects a running
    /// stopwatch.
    GuardNew,
}

impl Error {
    /// Returns `true` if the [`Stopwatch`](crate::Stopwatch) was expected to be
    /// running.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Error;
    /// assert!(Error::SwStop.expects_running());
    /// assert!(Error::GuardNew.expects_running());
    /// ```
    #[inline]
    #[must_use]
    pub const fn expects_running(&self) -> bool {
        match self {
            Self::SwStop | Self::GuardNew => true,
            Self::SwStart | Self::SwGuard => false,
        }
    }

    /// Returns `true` if the [`Stopwatch`](crate::Stopwatch) was expected to be
    /// stopped.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Error;
    /// assert!(Error::SwStart.expects_stopped());
    /// assert!(Error::SwGuard.expects_stopped());
    /// ```
    #[inline]
    #[must_use]
    pub const fn expects_stopped(&self) -> bool {
        !self.expects_running()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), fmt::Error> {
        #[allow(clippy::inline_always)]
        #[inline(always)]
        const fn state_to_str(running: bool) -> &'static str {
            if running {
                "running"
            } else {
                "stopped"
            }
        }

        let verb = match self {
            Self::SwStart => "started stopwatch",
            Self::SwStop => "stopped stopwatch",
            Self::SwGuard => "guarded stopwatch",
            Self::GuardNew => "created stopwatch guard",
        };

        let state = state_to_str(!self.expects_running());
        let expected_state = state_to_str(self.expects_running());

        write!(
            f,
            "{} while {}, but expected {}",
            verb, state, expected_state,
        )
    }
}

impl error::Error for Error {}
