// libsw: stopwatch library
// copyright (C) 2022  Ula Shipman <ula.hello@mailbox.org>
// licensed under MIT OR GPL-3.0-or-later

use crate::Guard;

use core::fmt;
use core::ops;
use core::time::Duration;
use std::error;
use std::time::Instant;

/// A `Stopwatch` measures and accumulates elapsed time between starts and
/// stops.
///
/// # Implementation details
///
/// Internally, a `Stopwatch` combines a saved elapsed time and an instant which
/// records the latest start time.
///
/// While the start time is [`Some`], the `Stopwatch` is running. When it stops,
/// the time which has elapsed since the start time is added to the elapsed
/// time, and the start time is set to [`None`].
#[derive(Clone, Copy, Debug, Hash)]
pub struct Stopwatch {
    elapsed: Duration,
    start: Option<Instant>,
}

impl Stopwatch {
    /// Returns a stopped `Stopwatch` with zero elapsed time.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Stopwatch;
    /// # use core::time::Duration;
    /// # fn main() {
    /// let sw = Stopwatch::new();
    /// assert!(sw.is_stopped());
    /// assert_eq!(sw.elapsed(), Duration::ZERO);
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self::with_elapsed(Duration::ZERO)
    }

    /// Returns a running `Stopwatch` initialized with zero elapsed time.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Stopwatch;
    /// # fn main() {
    /// let sw = Stopwatch::new_started();
    /// // if we measure sw.elapsed(), it very likely is *not* zero, since it's already running.
    /// assert!(sw.is_running());
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub fn new_started() -> Self {
        Self::with_elapsed_started(Duration::ZERO)
    }

    /// Returns a stopped `Stopwatch` with the given elapsed time.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Stopwatch;
    /// # use core::time::Duration;
    /// # fn main() {
    /// let sw = Stopwatch::with_elapsed(Duration::from_secs(1));
    /// assert!(sw.is_stopped());
    /// assert_eq!(sw.elapsed(), Duration::from_secs(1));
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub const fn with_elapsed(elapsed: Duration) -> Self {
        Self::from_raw(elapsed, None)
    }

    /// Returns a running `Stopwatch` initialized with the given elapsed time.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Stopwatch;
    /// # use core::time::Duration;
    /// # fn main() {
    /// let sw = Stopwatch::with_elapsed_started(Duration::from_secs(1));
    /// assert!(sw.is_running());
    /// assert!(sw.elapsed() >= Duration::from_secs(1));
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub fn with_elapsed_started(elapsed: Duration) -> Self {
        Self::from_raw(elapsed, Some(Instant::now()))
    }

    /// Returns a `Stopwatch` from its raw parts.
    ///
    /// See the [struct documentation](Self) for details about `Stopwatch`
    /// internals.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Stopwatch;
    /// # use core::time::Duration;
    /// # fn main() {
    /// let sw = Stopwatch::from_raw(Duration::from_secs(1), None);
    /// assert!(sw.is_stopped());
    /// assert_eq!(sw.elapsed(), Duration::from_secs(1));
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub const fn from_raw(elapsed: Duration, start: Option<Instant>) -> Self {
        Self { elapsed, start }
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
    /// let sw = Stopwatch::new_started();
    /// thread::sleep(Duration::from_millis(100));
    /// assert!(sw.elapsed() >= Duration::from_millis(100));
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub fn elapsed(&self) -> Duration {
        self.elapsed_at(Instant::now())
    }

    /// Returns the total time elapsed, measured at the given [`Instant`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Stopwatch;
    /// # use std::time::Instant;
    /// # fn main() {
    /// let sw_1 = Stopwatch::new_started();
    /// let sw_2 = sw_1.clone();
    /// let anchor = Instant::now();
    /// assert!(sw_1.elapsed_at(anchor) == sw_2.elapsed_at(anchor));
    /// # }
    /// ```
    #[must_use]
    pub fn elapsed_at(&self, anchor: Instant) -> Duration {
        if let Some(start) = self.start {
            self.elapsed
                .saturating_add(anchor.saturating_duration_since(start))
        } else {
            self.elapsed
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
    /// let mut sw = Stopwatch::new();
    /// assert!(sw.start().is_ok());
    /// assert!(sw.start().is_err());
    ///
    /// let then = sw.elapsed();
    /// thread::sleep(Duration::from_millis(100));
    /// let now = sw.elapsed();
    /// assert!(then != now);
    /// # }
    /// ```
    #[inline]
    pub fn start(&mut self) -> Result<(), Error> {
        self.start_at(Instant::now())
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
    /// let mut sw = Stopwatch::new_started();
    /// assert!(sw.stop().is_ok());
    /// assert!(sw.stop().is_err());
    ///
    /// let then = sw.elapsed();
    /// thread::sleep(Duration::from_millis(100));
    /// let now = sw.elapsed();
    /// assert!(then == now);
    /// # }
    /// ```
    #[inline]
    pub fn stop(&mut self) -> Result<(), Error> {
        self.stop_at(Instant::now())
    }

    /// Starts measuring the time elapsed at the given [`Instant`].
    ///
    /// # Errors
    ///
    /// Returns [`AlreadyStarted`](Error::AlreadyStarted) if the stopwatch is
    /// running.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::{Error, Stopwatch};
    /// # use core::time::Duration;
    /// # use std::thread;
    /// # use std::time::Instant;
    /// # fn main() -> Result<(), Error> {
    /// let mut sw_1 = Stopwatch::new();
    /// let mut sw_2 = Stopwatch::new();
    ///
    /// let start = Instant::now();
    /// // off to the races! at the same time!
    /// sw_1.start_at(start)?;
    /// sw_2.start_at(start)?;
    ///
    /// thread::sleep(Duration::from_millis(100));
    /// let anchor = Instant::now();
    ///
    /// assert_eq!(sw_1.elapsed_at(anchor), sw_2.elapsed_at(anchor)); // 'twas a tie
    /// assert!(sw_1.elapsed_at(anchor) >= Duration::from_millis(100));
    /// # Ok(())
    /// # }
    /// ```
    pub fn start_at(&mut self, anchor: Instant) -> Result<(), Error> {
        if self.is_running() {
            Err(Error::AlreadyStarted)
        } else {
            self.start = Some(anchor);
            Ok(())
        }
    }

    /// Stops measuring the time elapsed since the last start, at the given
    /// [`Instant`]. If `anchor` is earlier than the last start, there is no
    /// effect on the elapsed time.
    ///
    /// # Errors
    ///
    /// Returns [`AlreadyStopped`](Error::AlreadyStopped) if the stopwatch is
    /// not running.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::{Error, Stopwatch};
    /// # use core::time::Duration;
    /// # use std::thread;
    /// # use std::time::Instant;
    /// # fn main() -> Result<(), Error> {
    /// let mut sw_1 = Stopwatch::new_started();
    /// let mut sw_2 = sw_1.clone();
    /// let stop = Instant::now();
    /// sw_1.stop_at(stop)?;
    /// sw_2.stop_at(stop)?;
    /// assert_eq!(sw_1.elapsed(), sw_2.elapsed());
    /// # Ok(())
    /// # }
    /// ```
    pub fn stop_at(&mut self, anchor: Instant) -> Result<(), Error> {
        if let Some(start) = self.start {
            *self += anchor.saturating_duration_since(start);
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
    /// let mut sw = Stopwatch::new();
    /// sw.toggle();
    /// assert!(sw.is_running());
    /// sw.toggle();
    /// assert!(sw.is_stopped());
    /// # }
    /// ```
    #[inline]
    pub fn toggle(&mut self) {
        if self.stop().is_err() {
            self.start = Some(Instant::now());
        }
    }

    /// Starts the `Stopwatch`, returning a [`Guard`] which when dropped, will
    /// stop the `Stopwatch`.
    ///
    /// For examples on how to use `Guard`s, see the [struct
    /// documentation](Guard).
    ///
    /// # Errors
    ///
    /// Returns [`AlreadyStarted`](Error::AlreadyStarted) if the stopwatch is
    /// running.
    pub fn guard(&mut self) -> Result<Guard<'_>, Error> {
        self.start()?;
        Ok(Guard { inner: self })
    }

    /// Starts the `Stopwatch` at the given [`Instant`], returning a [`Guard`]
    /// which when dropped, will stop the `Stopwatch`.
    ///
    /// For details about `anchor`, see [`start_at`](Self::start_at). For
    /// examples on how to use `Guard`s, see the [struct
    /// documentation](Guard).
    ///
    /// # Errors
    ///
    /// Returns [`AlreadyStarted`](Error::AlreadyStarted) if the stopwatch is
    /// running.
    pub fn guard_at(&mut self, anchor: Instant) -> Result<Guard<'_>, Error> {
        self.start_at(anchor)?;
        Ok(Guard { inner: self })
    }

    /// Returns `true` if the stopwatch is running.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Stopwatch;
    /// # fn main() {
    /// let sw = Stopwatch::new_started();
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
    /// let sw = Stopwatch::new();
    /// assert!(sw.is_stopped());
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_stopped(&self) -> bool {
        self.start.is_none()
    }

    /// Stops and resets the elapsed time to zero.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Stopwatch;
    /// # use core::time::Duration;
    /// # fn main() {
    /// let mut sw = Stopwatch::with_elapsed_started(Duration::from_secs(1));
    /// sw.reset();
    /// assert_eq!(sw.elapsed(), Duration::ZERO);
    /// assert!(sw.is_stopped());
    /// # }
    /// ```
    #[inline]
    pub fn reset(&mut self) {
        *self = Self::new();
    }

    /// Stops and sets the total elapsed time to `new`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Stopwatch;
    /// # use core::time::Duration;
    /// # fn main() {
    /// let mut sw = Stopwatch::new();
    /// sw.set(Duration::from_secs(1));
    /// assert_eq!(sw.elapsed(), Duration::from_secs(1));
    /// # }
    /// ```
    #[inline]
    pub fn set(&mut self, new: Duration) {
        *self = Self::with_elapsed(new);
    }

    /// Stops and sets the total elapsed time to `new`, returning the previous
    /// elapsed time.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Stopwatch;
    /// # use core::time::Duration;
    /// # fn main() {
    /// let mut sw = Stopwatch::with_elapsed(Duration::from_secs(3));
    /// let previous = sw.replace(Duration::from_secs(1));
    /// assert_eq!(previous, Duration::from_secs(3));
    /// assert_eq!(sw.elapsed(), Duration::from_secs(1));
    /// # }
    /// ```
    pub fn replace(&mut self, new: Duration) -> Duration {
        let old = self.elapsed();
        self.set(new);
        old
    }

    /// Adds `dur` to the total elapsed time. If overflow occurred, the total
    /// elapsed time is set to [`Duration::MAX`].
    ///
    /// ```
    /// # use libsw::Stopwatch;
    /// # use core::time::Duration;
    /// # fn main() {
    /// let mut sw = Stopwatch::with_elapsed(Duration::from_secs(1));
    /// sw = sw.saturating_add(Duration::from_secs(1));
    /// assert_eq!(sw.elapsed(), Duration::from_secs(2));
    /// sw = sw.saturating_add(Duration::MAX);
    /// assert_eq!(sw.elapsed(), Duration::MAX);
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub const fn saturating_add(mut self, dur: Duration) -> Self {
        self.elapsed = self.elapsed.saturating_add(dur);
        self
    }

    /// Subtracts `dur` from the total elapsed time. If overflow occurred, the
    /// total elapsed time is set to [`Duration::ZERO`].
    ///
    /// ```
    /// # use libsw::Stopwatch;
    /// # use core::time::Duration;
    /// # fn main() {
    /// let mut sw = Stopwatch::with_elapsed(Duration::from_secs(1));
    /// sw = sw.saturating_sub(Duration::from_secs(1));
    /// assert_eq!(sw.elapsed(), Duration::ZERO);
    /// sw = sw.saturating_sub(Duration::from_secs(1));
    /// assert_eq!(sw.elapsed(), Duration::ZERO);
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub fn saturating_sub(mut self, dur: Duration) -> Self {
        self.sync_elapsed();
        self.elapsed = self.elapsed.saturating_sub(dur);
        self
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
    /// Returns the default `Stopwatch`. Same as calling [`Stopwatch::new`].
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl ops::Add<Duration> for Stopwatch {
    type Output = Self;

    /// Alias to [`Stopwatch::saturating_add`].
    #[inline]
    fn add(self, rhs: Duration) -> Self::Output {
        self.saturating_add(rhs)
    }
}

impl ops::Sub<Duration> for Stopwatch {
    type Output = Self;

    /// Alias to [`Stopwatch::saturating_sub`].
    #[inline]
    fn sub(self, rhs: Duration) -> Self::Output {
        self.saturating_sub(rhs)
    }
}

impl ops::AddAssign<Duration> for Stopwatch {
    #[inline]
    fn add_assign(&mut self, rhs: Duration) {
        *self = *self + rhs;
    }
}

impl ops::SubAssign<Duration> for Stopwatch {
    #[inline]
    fn sub_assign(&mut self, rhs: Duration) {
        *self = *self - rhs;
    }
}

/// Errors associated with [`Stopwatch`]
///
/// # Examples
///
/// ```
/// # use libsw::{Error, Stopwatch};
/// # fn main() -> Result<(), Error> {
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str(match self {
            Self::AlreadyStarted => "stopwatch already started",
            Self::AlreadyStopped => "stopwatch already stopped",
        })
    }
}

impl error::Error for Error {}
