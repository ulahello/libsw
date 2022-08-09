// libsw: stopwatch library
// copyright (C) 2022  Ula Shipman <ula.hello@mailbox.org>
// licensed under MIT OR GPL-3.0-or-later

use core::fmt;
use core::ops;
use core::time::Duration;
use std::error;
use std::time::Instant;

/// Stopwatch abstraction
///
/// Measures and accumulates time between starts and stops.
#[derive(Clone, Copy, Debug)]
pub struct Stopwatch {
    elapsed: Duration,
    start: Option<Instant>,
}

impl Stopwatch {
    /// Returns a [`Stopwatch`] with the given elapsed time. If `running`, start
    /// the stopwatch.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Stopwatch;
    /// # use core::time::Duration;
    /// # fn main() {
    /// let sw = Stopwatch::new(Duration::from_secs(1), false);
    /// assert!(sw.is_stopped());
    /// assert_eq!(sw.elapsed(), Duration::from_secs(1));
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub fn new(elapsed: Duration, running: bool) -> Self {
        Self {
            elapsed,
            start: if running { Some(Instant::now()) } else { None },
        }
    }

    /// Starts measuring the time elapsed.
    ///
    /// # Errors
    ///
    /// Returns [`AlreadyStarted`](Error::AlreadyStarted) if the stopwatch is
    /// running.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Stopwatch;
    /// # use core::time::Duration;
    /// # use std::thread;
    /// # fn main() {
    /// let mut sw = Stopwatch::default();
    /// assert!(sw.start().is_ok());
    /// assert!(sw.start().is_err());
    ///
    /// let then = sw.elapsed();
    /// thread::sleep(Duration::from_millis(100));
    /// let now = sw.elapsed();
    /// assert!(then != now);
    /// # }
    /// ```
    pub fn start(&mut self) -> Result<(), Error> {
        if self.is_running() {
            Err(Error::AlreadyStarted)
        } else {
            self.start = Some(Instant::now());
            Ok(())
        }
    }

    /// Stops measuring the time elapsed since the last start.
    ///
    /// # Errors
    ///
    /// Returns [`AlreadyStopped`](Error::AlreadyStopped) if the stopwatch is
    /// not running.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Stopwatch;
    /// # use core::time::Duration;
    /// # use std::thread;
    /// # fn main() {
    /// let mut sw = Stopwatch::new(Duration::ZERO, true);
    /// assert!(sw.stop().is_ok());
    /// assert!(sw.stop().is_err());
    ///
    /// let then = sw.elapsed();
    /// thread::sleep(Duration::from_millis(100));
    /// let now = sw.elapsed();
    /// assert!(then == now);
    /// # }
    /// ```
    pub fn stop(&mut self) -> Result<(), Error> {
        if let Some(start) = self.start {
            *self += Instant::now().saturating_duration_since(start);
            self.start = None;
            Ok(())
        } else {
            Err(Error::AlreadyStopped)
        }
    }

    /// Toggles whether the stopwatch is running or stopped.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Stopwatch;
    /// # fn main() {
    /// let mut sw = Stopwatch::default();
    /// sw.toggle();
    /// assert!(sw.is_running());
    /// sw.toggle();
    /// assert!(sw.is_stopped());
    /// # }
    /// ```
    #[inline]
    pub fn toggle(&mut self) {
        if self.is_running() {
            let _ = self.stop();
        } else {
            let _ = self.start();
        }
    }

    /// Stops and resets the elapsed time to zero.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Stopwatch;
    /// # use core::time::Duration;
    /// # fn main() {
    /// let mut sw = Stopwatch::new(Duration::from_secs(1), true);
    /// sw.reset();
    /// assert_eq!(sw.elapsed(), Duration::ZERO);
    /// assert!(sw.is_stopped());
    /// # }
    /// ```
    #[inline]
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Stops and sets the total elapsed time to `new`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Stopwatch;
    /// # use core::time::Duration;
    /// # fn main() {
    /// let mut sw = Stopwatch::default();
    /// sw.set(Duration::from_secs(1));
    /// assert_eq!(sw.elapsed(), Duration::from_secs(1));
    /// # }
    /// ```
    #[inline]
    pub fn set(&mut self, new: Duration) {
        self.elapsed = new;
        self.start = None;
    }

    /// Returns the total time elapsed.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::{Error, Stopwatch};
    /// # use core::time::Duration;
    /// # use std::thread;
    /// # fn main() -> Result<(), Error> {
    /// let mut sw = Stopwatch::default();
    /// sw.start()?;
    /// thread::sleep(Duration::from_millis(100));
    /// assert!(sw.elapsed() >= Duration::from_millis(100));
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn elapsed(&self) -> Duration {
        if let Some(start) = self.start {
            self.elapsed
                .saturating_add(Instant::now().saturating_duration_since(start))
        } else {
            self.elapsed
        }
    }

    /// Returns `true` if the stopwatch is running.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Stopwatch;
    /// # use core::time::Duration;
    /// # fn main() {
    /// let sw = Stopwatch::new(Duration::ZERO, true);
    /// assert!(sw.is_running());
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_running(&self) -> bool {
        self.start.is_some()
    }

    /// Returns `true` if the stopwatch is stopped.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Stopwatch;
    /// # fn main() {
    /// let sw = Stopwatch::default();
    /// assert!(sw.is_stopped());
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_stopped(&self) -> bool {
        self.start.is_none()
    }

    /// Syncs changes in the elapsed time, effectively toggling the stopwatch
    /// twice.
    #[inline]
    fn sync_elapsed(&mut self) {
        if let Some(start) = self.start {
            let now = Instant::now();
            *self += now.saturating_duration_since(start);
            self.start = Some(now);
        }
    }
}

impl Default for Stopwatch {
    /// Returns a stopped [`Stopwatch`] with zero elapsed time.
    #[inline]
    fn default() -> Self {
        Self::new(Duration::ZERO, false)
    }
}

impl ops::Add<Duration> for Stopwatch {
    type Output = Self;

    #[inline]
    fn add(mut self, rhs: Duration) -> Self::Output {
        self += rhs;
        self
    }
}

impl ops::Sub<Duration> for Stopwatch {
    type Output = Self;

    #[inline]
    fn sub(mut self, rhs: Duration) -> Self::Output {
        self -= rhs;
        self
    }
}

impl ops::AddAssign<Duration> for Stopwatch {
    /// Adds `rhs` to the total elapsed time.
    ///
    /// ```
    /// # use libsw::Stopwatch;
    /// # use core::time::Duration;
    /// # fn main() {
    /// let mut sw = Stopwatch::new(Duration::from_secs(1), false);
    /// sw += Duration::from_secs(1);
    /// assert_eq!(sw.elapsed(), Duration::from_secs(2));
    /// # }
    /// ```
    #[inline]
    fn add_assign(&mut self, rhs: Duration) {
        self.elapsed = self.elapsed.saturating_add(rhs);
    }
}

impl ops::SubAssign<Duration> for Stopwatch {
    /// Subtracts `rhs` from the total elapsed time.
    ///
    /// ```
    /// # use libsw::Stopwatch;
    /// # use core::time::Duration;
    /// # fn main() {
    /// let mut sw = Stopwatch::new(Duration::from_secs(1), false);
    /// sw -= Duration::from_secs(1);
    /// assert_eq!(sw.elapsed(), Duration::ZERO);
    /// # }
    /// ```
    #[inline]
    fn sub_assign(&mut self, rhs: Duration) {
        self.sync_elapsed();
        self.elapsed = self.elapsed.saturating_sub(rhs);
    }
}

/// Errors associated with [`Stopwatch`]
///
/// # Examples
///
/// ```
/// # use libsw::{Error, Stopwatch};
/// # fn main() -> Result<(), Error> {
/// let mut sw = Stopwatch::default();
/// sw.start()?;
/// assert_eq!(sw.start(), Err(Error::AlreadyStarted));
/// sw.stop()?;
/// assert_eq!(sw.stop(), Err(Error::AlreadyStopped));
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Error {
    /// Called [`Stopwatch::start`] while running
    AlreadyStarted,
    /// Called [`Stopwatch::stop`] while stopped
    AlreadyStopped,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str(match self {
            Self::AlreadyStarted => "stopwatch already started",
            Self::AlreadyStopped => "stopwatch already stopped",
        })
    }
}

impl error::Error for Error {}
