// libsw: stopwatch library
// copyright (C) 2022-2023 Ula Shipman <ula.hello@mailbox.org>
// licensed under MIT OR Apache-2.0

/* TODO: weird to have stopwatch errors */

use core::fmt;

/// Alias to `Result<T, Error>`.
pub type Result<T> = ::core::result::Result<T, Error>;

/// Enumeration over possible errors.
///
/// # `Display` implementation
///
/// `Error` implements [`Display`](core::fmt::Display) by explaining what
/// happened and what was expected instead.
///
/// The exact text of these explanations is not stable and should not be relied
/// upon.
///
/// ```
/// # use libsw::Error;
/// let error = Error::SwStart;
/// assert_eq!(
///     error.to_string(),
///     "started stopwatch while running, but expected stopped"
/// );
/// ```
///
/// # Feature flags
///
/// When the `std` feature is enabled, `Error` implements [`std::error::Error`].
///
/// When the `nightly` feature is enabled and the `std` feature is **not**
/// enabled, `Error` implements [`core::error::Error`]. This requires a nightly
/// compiler.
///
/// # Examples
///
/// ```
/// # use libsw::{Error, Result, Sw};
/// # fn main() -> Result<()> {
/// let mut sw = Sw::new_started();
/// assert_eq!(sw.start(), Err(Error::SwStart));
/// sw.stop()?;
/// assert_eq!(sw.stop(), Err(Error::SwStop));
/// # Ok(())
/// # }
/// ```
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Error {
    /// Returned by methods that [start](crate::StopwatchImpl::start) the
    /// stopwatch. Expects that it's stopped.
    SwStart,

    /// Returned by methods that [stop](crate::StopwatchImpl::stop) the
    /// stopwatch. Expects that it's running.
    SwStop,

    /// Returned by methods that [guard](crate::StopwatchImpl::guard) the
    /// stopwatch. Expects that it's stopped.
    SwGuard,

    /// Returned by [`Guard::new`](crate::Guard::new). Expects that it's running.
    GuardNew,
}

impl Error {
    /// Returns `true` if the [stopwatch](crate::StopwatchImpl) was expected to
    /// be running.
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

    /// Returns `true` if the [stopwatch](crate::StopwatchImpl) was expected to
    /// be stopped.
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[inline]
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

        write!(f, "{verb} while {state}, but expected {expected_state}")
    }
}

#[cfg(feature = "std")]
impl ::std::error::Error for Error {}

#[cfg(all(feature = "nightly", not(feature = "std")))]
impl ::core::error::Error for Error {}
