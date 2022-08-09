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
///
/// # Implementations
///
/// [`Stopwatch`] implements [`Default`], returning a stopped stopwatch with
/// zero time elapsed.
///
/// Also, it implements [`Add`](ops::Add), [`Sub`](ops::Sub),
/// [`AddAssign`](ops::AddAssign), and [`SubAssign`](ops::SubAssign). These
/// methods will add or subtract the right-hand-side duration from the
/// stopwatch's elapsed time.
#[derive(Clone, Copy, Debug)]
pub struct Stopwatch {
    elapsed: Duration,
    start: Option<Instant>,
}

impl Stopwatch {
    /// Create a [`Stopwatch`] with the given elapsed time. If `running`, start
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

    /// Start measuring the time elapsed.
    ///
    /// # Errors
    ///
    /// Returns [`Error::AlreadyStarted`] if the stopwatch is running.
    pub fn start(&mut self) -> Result<(), Error> {
        if self.is_running() {
            Err(Error::AlreadyStarted)
        } else {
            self.start = Some(Instant::now());
            Ok(())
        }
    }

    /// Stop measuring the time elapsed since the last start.
    ///
    /// # Errors
    ///
    /// Returns [`Error::AlreadyStopped`] if the stopwatch is not running.
    pub fn stop(&mut self) -> Result<(), Error> {
        if let Some(start) = self.start {
            *self += Instant::now().saturating_duration_since(start);
            self.start = None;
            Ok(())
        } else {
            Err(Error::AlreadyStopped)
        }
    }

    /// Toggle whether the stopwatch is running or stopped.
    ///
    /// If stopped, then start, and if running, then stop.
    #[inline]
    pub fn toggle(&mut self) {
        if self.is_running() {
            let _ = self.stop();
        } else {
            let _ = self.start();
        }
    }

    /// Stop and reset the elapsed time to zero.
    #[inline]
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Stop and set the total elapsed time to `new`.
    #[inline]
    pub fn set(&mut self, new: Duration) {
        self.elapsed = new;
        self.start = None;
    }

    /// Return the total time elapsed.
    #[must_use]
    pub fn elapsed(&self) -> Duration {
        if let Some(start) = self.start {
            self.elapsed
                .saturating_add(Instant::now().saturating_duration_since(start))
        } else {
            self.elapsed
        }
    }

    /// Return whether the stopwatch is running.
    #[inline]
    #[must_use]
    pub const fn is_running(&self) -> bool {
        self.start.is_some()
    }

    /// Return whether the stopwatch is stopped.
    #[inline]
    #[must_use]
    pub const fn is_stopped(&self) -> bool {
        self.start.is_none()
    }

    /// Sync changes in the elapsed time, effectively toggling the stopwatch
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
    #[inline]
    fn add_assign(&mut self, rhs: Duration) {
        self.elapsed = self.elapsed.saturating_add(rhs);
    }
}

impl ops::SubAssign<Duration> for Stopwatch {
    #[inline]
    fn sub_assign(&mut self, rhs: Duration) {
        self.sync_elapsed();
        self.elapsed = self.elapsed.saturating_sub(rhs);
    }
}

/// Errors associated with [`Stopwatch`]
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
