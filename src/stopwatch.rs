// libsw: stopwatch library
// copyright (C) 2022-2023 Ula Shipman <ula.hello@mailbox.org>
// licensed under MIT OR Apache-2.0

use crate::{Error, Guard, Instant};

use core::hash::{Hash, Hasher};
use core::ops;
use core::time::Duration;

/// A stopwatch measures and accumulates elapsed time between starts and stops.
///
/// Stopwatches work with any type that implements [`Instant`].
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Copy, Debug, Eq)]
pub struct StopwatchImpl<I: Instant> {
    elapsed: Duration,
    start: Option<I>,
}

impl<I: Instant> StopwatchImpl<I> {
    /// Returns a stopped stopwatch with zero elapsed time.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Stopwatch;
    /// # use core::time::Duration;
    /// let sw = Stopwatch::new();
    /// assert!(sw.is_stopped());
    /// assert_eq!(sw.elapsed(), Duration::ZERO);
    /// ```
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self::with_elapsed(Duration::ZERO)
    }

    /// Returns a running stopwatch initialized with zero elapsed time.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Stopwatch;
    /// let sw = Stopwatch::new_started();
    /// assert!(sw.is_running());
    /// ```
    #[inline]
    #[must_use]
    pub fn new_started() -> Self {
        Self::with_elapsed_started(Duration::ZERO)
    }

    /// Returns a stopwatch initialized with zero elapsed time, started at the
    /// given instant.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Stopwatch;
    /// # use std::time::Instant;
    /// let now = Instant::now();
    /// let sw_1 = Stopwatch::new_started_at(now);
    /// let sw_2 = Stopwatch::new_started_at(now);
    /// // they've both started at the same time
    /// assert_eq!(sw_1, sw_2);
    /// ```
    #[inline]
    #[must_use]
    pub const fn new_started_at(start: I) -> Self {
        Self::from_raw(Duration::ZERO, Some(start))
    }

    /// Returns a stopped stopwatch with the given elapsed time.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Stopwatch;
    /// # use core::time::Duration;
    /// let sw = Stopwatch::with_elapsed(Duration::from_secs(1));
    /// assert!(sw.is_stopped());
    /// assert_eq!(sw.elapsed(), Duration::from_secs(1));
    /// ```
    #[inline]
    #[must_use]
    pub const fn with_elapsed(elapsed: Duration) -> Self {
        Self::from_raw(elapsed, None)
    }

    /// Returns a running stopwatch initialized with the given elapsed time.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Stopwatch;
    /// # use core::time::Duration;
    /// let sw = Stopwatch::with_elapsed_started(Duration::from_secs(1));
    /// assert!(sw.is_running());
    /// assert!(sw.elapsed() >= Duration::from_secs(1));
    /// ```
    #[inline]
    #[must_use]
    pub fn with_elapsed_started(elapsed: Duration) -> Self {
        Self::from_raw(elapsed, Some(I::now()))
    }

    /// Returns a stopwatch from its raw parts.
    ///
    /// Internally, a `StopwatchImpl` combines a saved elapsed time and an
    /// instant which records the latest start time.
    ///
    /// While the start time is [`Some`], the stopwatch is running. When it
    /// stops, the time which has elapsed since the start time is added to the
    /// elapsed time, and the start time is set to [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Stopwatch;
    /// # use core::time::Duration;
    /// let sw = Stopwatch::from_raw(Duration::from_secs(1), None);
    /// assert!(sw.is_stopped());
    /// assert_eq!(sw.elapsed(), Duration::from_secs(1));
    /// ```
    ///
    /// # Notes
    ///
    /// It is possible to craft two stopwatches whose internal components
    /// differ, but are equal according to [`PartialEq`], [`Eq`], and [`Hash`].
    ///
    /// ```
    /// # use libsw::Stopwatch;
    /// # use core::time::Duration;
    /// # use std::time::Instant;
    /// let mut elapsed = Duration::from_secs(10);
    /// let mut start = Instant::now();
    /// let sw_1 = Stopwatch::from_raw(elapsed, Some(start));
    /// let sw_2 = Stopwatch::from_raw(
    ///     elapsed - Duration::from_secs(1),     // now `elapsed()` is 1s less
    ///     Some(start - Duration::from_secs(1)), // now with start pushed back, `elapsed()` is equal
    /// );
    ///
    /// // different components, but they are equal!
    /// assert_eq!(sw_1, sw_2);
    /// ```
    #[inline]
    #[must_use]
    pub const fn from_raw(elapsed: Duration, start: Option<I>) -> Self {
        Self { elapsed, start }
    }

    /// Returns `true` if the stopwatch is running.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Stopwatch;
    /// let sw = Stopwatch::new_started();
    /// assert!(sw.is_running());
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
    /// let sw = Stopwatch::new();
    /// assert!(sw.is_stopped());
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_stopped(&self) -> bool {
        !self.is_running()
    }

    /// Returns the total time elapsed. If overflow occurs, the elapsed time is
    /// saturated to [`Duration::MAX`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Stopwatch;
    /// # use core::time::Duration;
    /// # use std::thread;
    /// # fn main() -> libsw::Result<()> {
    /// let sw = Stopwatch::new_started();
    /// thread::sleep(Duration::from_millis(100));
    /// assert!(sw.elapsed() >= Duration::from_millis(100));
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub fn elapsed(&self) -> Duration {
        self.elapsed_at(I::now())
    }

    /// Returns the total time elapsed, measured as if the current time were
    /// `anchor`. If overflow occurs, the elapsed time is saturated to
    /// [`Duration::MAX`].
    ///
    /// # Notes
    ///
    /// `anchor` saturates to the last instant the stopwatch was started.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Stopwatch;
    /// # use std::time::Instant;
    /// let sw_1 = Stopwatch::new_started();
    /// let sw_2 = sw_1.clone();
    /// let anchor = Instant::now();
    /// assert!(sw_1.elapsed_at(anchor) == sw_2.elapsed_at(anchor));
    /// ```
    #[inline]
    #[must_use]
    pub fn elapsed_at(&self, anchor: I) -> Duration {
        self.checked_elapsed_at(anchor).unwrap_or(Duration::MAX)
    }

    /// Computes the total time elapsed. If overflow occurred, returns [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Stopwatch;
    /// # use core::time::Duration;
    /// # use std::thread;
    /// # fn main() -> libsw::Result<()> {
    /// let mut sw = Stopwatch::new_started();
    /// thread::sleep(Duration::from_millis(100));
    /// assert!(sw.checked_elapsed().unwrap() >= Duration::from_millis(100));
    /// sw += Duration::MAX;
    /// assert!(sw.checked_elapsed().is_none());
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_elapsed(&self) -> Option<Duration> {
        self.checked_elapsed_at(I::now())
    }

    /// Computes the total time elapsed, measured as if the current time were
    /// `anchor`. If overflow occurred, returns [`None`].
    ///
    /// # Notes
    ///
    /// `anchor` saturates to the last instant the stopwatch was started.
    ///
    /// # Examples
    ///
    /// See the documentation for [`checked_elapsed`](Self::checked_elapsed) for
    /// a related example.
    #[must_use]
    pub fn checked_elapsed_at(&self, anchor: I) -> Option<Duration> {
        self.start.map_or(Some(self.elapsed), |start| {
            self.elapsed
                .checked_add(anchor.saturating_duration_since(start))
        })
    }

    /// Starts measuring the time elapsed.
    ///
    /// # Errors
    ///
    /// Returns [`SwStart`](Error::SwStart) if the stopwatch is running.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Stopwatch;
    /// # use core::time::Duration;
    /// # use std::thread;
    /// let mut sw = Stopwatch::new();
    /// assert!(sw.start().is_ok());
    /// assert!(sw.start().is_err());
    ///
    /// let then = sw.elapsed();
    /// thread::sleep(Duration::from_millis(100));
    /// let now = sw.elapsed();
    /// assert!(then != now);
    /// ```
    #[inline]
    pub fn start(&mut self) -> crate::Result<()> {
        self.start_at(I::now())
    }

    /// Stops measuring the time elapsed since the last start.
    ///
    /// # Errors
    ///
    /// Returns [`SwStop`](Error::SwStop) if the stopwatch is already stopped.
    ///
    /// # Notes
    ///
    /// Overflows of the new elapsed time are saturated to [`Duration::MAX`].
    /// Use [`StopwatchImpl::checked_stop`] to explicitly check for overflow.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Stopwatch;
    /// # use core::time::Duration;
    /// # use std::thread;
    /// let mut sw = Stopwatch::new_started();
    /// assert!(sw.stop().is_ok());
    /// assert!(sw.stop().is_err());
    ///
    /// let then = sw.elapsed();
    /// thread::sleep(Duration::from_millis(100));
    /// let now = sw.elapsed();
    /// assert!(then == now);
    /// ```
    #[inline]
    pub fn stop(&mut self) -> crate::Result<()> {
        self.stop_at(I::now())
    }

    /// Starts measuring the time elapsed as if the current time were `anchor`.
    ///
    /// # Errors
    ///
    /// Returns [`SwStart`](Error::SwStart) if the stopwatch is running.
    ///
    /// # Notes
    ///
    /// If `anchor` is in the future, [`elapsed`](Self::elapsed) will return
    /// [`Duration::ZERO`] until the current time catches up to it.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Stopwatch;
    /// # use core::time::Duration;
    /// # use std::thread;
    /// # use std::time::Instant;
    /// # fn main() -> libsw::Result<()> {
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
    pub fn start_at(&mut self, anchor: I) -> crate::Result<()> {
        self.is_stopped()
            .then(|| self.start = Some(anchor))
            .ok_or(Error::SwStart)
    }

    /// Stops measuring the time elapsed since the last start as if the current
    /// time were `anchor`.
    ///
    /// # Errors
    ///
    /// Returns [`SwStop`](Error::SwStop) if the stopwatch is already stopped.
    ///
    /// # Notes
    ///
    /// - If `anchor` is earlier than the last start, there is no effect on the
    /// elapsed time.
    ///
    /// - Overflows of the new elapsed time are saturated to [`Duration::MAX`].
    /// Use [`StopwatchImpl::checked_stop_at`] to explicitly check for overflow.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Stopwatch;
    /// # use core::time::Duration;
    /// # use std::thread;
    /// # use std::time::Instant;
    /// # fn main() -> libsw::Result<()> {
    /// let mut sw_1 = Stopwatch::new_started();
    /// let mut sw_2 = sw_1.clone();
    /// let stop = Instant::now();
    /// sw_1.stop_at(stop)?;
    /// sw_2.stop_at(stop)?;
    /// assert_eq!(sw_1, sw_2);
    /// # Ok(())
    /// # }
    /// ```
    pub fn stop_at(&mut self, anchor: I) -> crate::Result<()> {
        self.start
            .take()
            .map(|start| *self += anchor.saturating_duration_since(start))
            .ok_or(Error::SwStop)
    }

    /// Tries to stop the stopwatch. If the new elapsed time overflows, returns
    /// [`None`] without mutating the stopwatch.
    ///
    /// # Errors
    ///
    /// Returns [`SwStop`](Error::SwStop) if the stopwatch is already stopped.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Stopwatch;
    /// # use core::time::Duration;
    /// # fn main() -> libsw::Result<()> {
    /// let mut sw = Stopwatch::new_started();
    /// assert!(sw.checked_stop()?.is_some());
    /// sw.set(Duration::MAX);
    /// sw.start()?;
    /// assert!(sw.checked_stop()?.is_none());
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn checked_stop(&mut self) -> crate::Result<Option<()>> {
        self.checked_stop_at(I::now())
    }

    /// Tries to stop the stopwatch, as if the current time were `anchor`. If
    /// the new elapsed time overflows, returns [`None`] without mutating the
    /// stopwatch.
    ///
    /// # Errors
    ///
    /// Returns [`SwStop`](Error::SwStop) if the stopwatch is already stopped.
    ///
    /// # Notes
    ///
    /// If `anchor` is earlier than the last start, there is no effect on the
    /// elapsed time.
    ///
    /// # Examples
    ///
    /// See [`StopwatchImpl::checked_stop`] for comparable example usage.
    pub fn checked_stop_at(&mut self, anchor: I) -> crate::Result<Option<()>> {
        self.start
            .map(|start| {
                self.checked_add(anchor.saturating_duration_since(start))
                    .map(|new| self.set(new.elapsed))
            })
            .ok_or(Error::SwStop)
    }

    /// Toggles whether the stopwatch is running or stopped.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Stopwatch;
    /// let mut sw = Stopwatch::new();
    /// sw.toggle();
    /// assert!(sw.is_running());
    /// sw.toggle();
    /// assert!(sw.is_stopped());
    /// ```
    #[inline]
    pub fn toggle(&mut self) {
        self.toggle_at(I::now());
    }

    /// Toggles whether the stopwatch is running or stopped, as if the current
    /// time were `anchor`.
    ///
    /// # Notes
    ///
    /// See [`start_at`](Self::start_at) and [`stop_at`](Self::stop_at) for
    /// notes about the chronology of `anchor`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Stopwatch;
    /// # use std::time::Instant;
    /// let mut left = Stopwatch::new();
    /// let mut right = Stopwatch::new_started();
    ///
    /// // perfect swap of left and right running
    /// let now = Instant::now();
    /// left.toggle_at(now);
    /// right.toggle_at(now);
    ///
    /// assert!(left.is_running());
    /// assert!(right.is_stopped());
    /// ```
    pub fn toggle_at(&mut self, anchor: I) {
        if self.stop_at(anchor).is_err() {
            let _ = self.start_at(anchor);
        }
    }

    /// Starts the stopwatch, returning a [`Guard`] which when dropped, will
    /// stop the stopwatch.
    ///
    /// # Errors
    ///
    /// Returns [`SwGuard`](Error::SwGuard) if the stopwatch is running.
    ///
    /// # Examples
    ///
    /// For examples on how to use `Guard`s, see the [struct
    /// documentation](Guard).
    #[inline]
    pub fn guard(&mut self) -> crate::Result<Guard<'_, I>> {
        self.guard_at(I::now())
    }

    /// Starts the stopwatch as if the current time were `anchor`, returning a
    /// [`Guard`], which when dropped, will stop the stopwatch.
    ///
    /// # Errors
    ///
    /// Returns [`SwGuard`](Error::SwGuard) if the stopwatch is running.
    ///
    /// # Notes
    ///
    /// For details about `anchor`, see [`start_at`](Self::start_at). For
    /// examples on how to use `Guard`s, see the [struct documentation](Guard).
    pub fn guard_at(&mut self, anchor: I) -> crate::Result<Guard<'_, I>> {
        self.start_at(anchor).map_err(|_| Error::SwGuard)?;
        let guard = Guard::new(self);
        debug_assert!(guard.is_ok());
        guard
    }

    /// Stops and resets the elapsed time to zero.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Stopwatch;
    /// # use core::time::Duration;
    /// let mut sw = Stopwatch::with_elapsed_started(Duration::from_secs(1));
    /// sw.reset();
    /// assert_eq!(sw, Stopwatch::new());
    /// ```
    #[inline]
    pub fn reset(&mut self) {
        *self = Self::new();
    }

    /// Resets the elapsed time to zero without affecting whether the stopwatch
    /// is running.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Stopwatch;
    /// # use core::time::Duration;
    /// # fn main() -> libsw::Result<()> {
    /// let mut sw = Stopwatch::with_elapsed_started(Duration::from_secs(1));
    /// sw.reset_in_place();
    /// assert!(sw.is_running());
    /// // new elapsed time is close to zero
    /// assert!(sw.elapsed() < Duration::from_millis(1));
    ///
    /// sw.stop()?;
    /// sw.reset_in_place();
    /// assert_eq!(sw, Stopwatch::new());
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn reset_in_place(&mut self) {
        self.reset_in_place_at(Instant::now());
    }

    /// Resets the elapsed time to zero without affecting whether the stopwatch
    /// is running.
    ///
    /// # Notes
    ///
    /// See [`start_at`](Self::start_at) for notes about the chronology of
    /// `anchor`.
    ///
    /// # Examples
    ///
    /// See the documentation for [`reset_in_place`](Self::reset_in_place) for a
    /// related example.
    #[inline]
    pub fn reset_in_place_at(&mut self, start: I) {
        self.set_in_place_at(Duration::ZERO, start);
    }

    /// Stops and sets the total elapsed time to `new`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Stopwatch;
    /// # use core::time::Duration;
    /// let mut sw = Stopwatch::new();
    /// sw.set(Duration::from_secs(1));
    /// assert_eq!(sw.elapsed(), Duration::from_secs(1));
    /// ```
    #[inline]
    pub fn set(&mut self, new: Duration) {
        *self = Self::with_elapsed(new);
    }

    /// Sets the total elapsed time to `new` without affecting whether the
    /// stopwatch is running.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Stopwatch;
    /// # use core::time::Duration;
    /// # fn main() -> libsw::Result<()> {
    /// let mut sw = Stopwatch::new();
    /// sw.set_in_place(Duration::from_secs(1));
    /// assert_eq!(sw.elapsed(), Duration::from_secs(1));
    /// assert!(sw.is_stopped());
    ///
    /// sw.start()?;
    /// sw.set_in_place(Duration::from_secs(2));
    /// assert!(sw.elapsed() >= Duration::from_secs(2));
    /// assert!(sw.is_running());
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn set_in_place(&mut self, new: Duration) {
        self.set_in_place_at(new, Instant::now());
    }

    /// Sets the total elapsed time to `new` as if the current time were
    /// `anchor`, and without affecting whether the stopwatch is running.
    ///
    /// # Notes
    ///
    /// See [`start_at`](Self::start_at) for notes about the chronology of
    /// `anchor`.
    ///
    /// # Examples
    ///
    /// See the documentation for [`set_in_place`](Self::set_in_place) for
    /// a related example.
    pub fn set_in_place_at(&mut self, new: Duration, anchor: I) {
        let was_running = self.is_running();
        self.set(new);
        if was_running {
            let result = self.start_at(anchor);
            debug_assert!(result.is_ok());
        }
    }

    /// Stops and sets the total elapsed time to `new`, returning the previous
    /// elapsed time.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Stopwatch;
    /// # use core::time::Duration;
    /// let mut sw = Stopwatch::with_elapsed(Duration::from_secs(3));
    /// let previous = sw.replace(Duration::from_secs(1));
    /// assert_eq!(previous, Duration::from_secs(3));
    /// assert_eq!(sw.elapsed(), Duration::from_secs(1));
    /// ```
    #[inline]
    pub fn replace(&mut self, new: Duration) -> Duration {
        self.replace_at(new, Instant::now())
    }

    /// Stops and sets the total elapsed time to `new`, returning the previous
    /// elapsed time as if the current time were `anchor`.
    ///
    /// # Notes
    ///
    /// See [`elapsed_at`](Self::elapsed_at) for notes about the chronology of
    /// `anchor`.
    ///
    /// # Examples
    ///
    /// See the documentation for [`replace`](Self::replace) for a related
    /// example.
    pub fn replace_at(&mut self, new: Duration, anchor: I) -> Duration {
        let old = self.elapsed_at(anchor);
        self.set(new);
        old
    }

    /// Adds `dur` to the total elapsed time. If overflow occurred, the total
    /// elapsed time is set to [`Duration::MAX`].
    ///
    /// ```
    /// # use libsw::Stopwatch;
    /// # use core::time::Duration;
    /// let mut sw = Stopwatch::with_elapsed(Duration::from_secs(1));
    /// sw = sw.saturating_add(Duration::from_secs(1));
    /// assert_eq!(sw.elapsed(), Duration::from_secs(2));
    /// sw = sw.saturating_add(Duration::MAX);
    /// assert_eq!(sw.elapsed(), Duration::MAX);
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
    /// let mut sw = Stopwatch::with_elapsed(Duration::from_secs(1));
    /// sw = sw.saturating_sub(Duration::from_secs(1));
    /// assert_eq!(sw.elapsed(), Duration::ZERO);
    /// sw = sw.saturating_sub(Duration::from_secs(1));
    /// assert_eq!(sw.elapsed(), Duration::ZERO);
    /// ```
    #[must_use]
    pub fn saturating_sub(mut self, dur: Duration) -> Self {
        self.sync_elapsed();
        self.elapsed = self.elapsed.saturating_sub(dur);
        self
    }

    /// Adds `dur` to the total elapsed time. If overflow occurred, returns
    /// [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Stopwatch;
    /// # use core::time::Duration;
    /// let mut sw = Stopwatch::new();
    /// sw = sw.checked_add(Duration::from_secs(1)).unwrap();
    /// assert_eq!(sw.elapsed(), Duration::from_secs(1));
    /// assert_eq!(sw.checked_add(Duration::MAX), None);
    /// ```
    #[must_use]
    pub const fn checked_add(mut self, dur: Duration) -> Option<Self> {
        match self.elapsed.checked_add(dur) {
            Some(new) => {
                self.elapsed = new;
                Some(self)
            }
            None => None,
        }
    }

    /// Subtracts `dur` from the total elapsed time. If overflow occurred,
    /// returns [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Stopwatch;
    /// # use core::time::Duration;
    /// let mut sw = Stopwatch::new();
    /// assert_eq!(sw.checked_sub(Duration::from_secs(1)), None);
    /// sw += Duration::from_secs(1);
    /// assert_eq!(
    ///     sw.checked_sub(Duration::from_secs(1)),
    ///     Some(Stopwatch::with_elapsed(Duration::ZERO)),
    /// );
    /// ```
    #[must_use]
    pub fn checked_sub(mut self, dur: Duration) -> Option<Self> {
        self.sync_elapsed();
        self.elapsed.checked_sub(dur).map(|new| {
            self.elapsed = new;
            self
        })
    }

    /// Syncs changes in the elapsed time, effectively toggling the stopwatch
    /// twice.
    #[inline] // fn is private; called in Self::saturating_sub and Self::checked_sub
    fn sync_elapsed(&mut self) {
        if let Some(start) = self.start {
            let now = I::now();
            *self += now.saturating_duration_since(start);
            self.start = Some(now);
        }
    }

    /// "Transfers" `elapsed` to `start`, such that [`Self::elapsed`] is
    /// unchanged, and the new `elapsed` is zero. Returns an error if the new
    /// start time cannot be represented.
    fn normalize_start(&mut self) -> Result<(), ()> {
        if let Some(ref mut instant) = self.start {
            if let Some(new) = instant.checked_sub(self.elapsed) {
                self.start = Some(new);
                self.elapsed = Duration::ZERO;
            } else {
                return Err(());
            }
        }
        Ok(())
    }
}

impl<I: Instant> Default for StopwatchImpl<I> {
    /// Returns the default stopwatch. Same as calling [`StopwatchImpl::new`].
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<I: Instant> ops::Add<Duration> for StopwatchImpl<I> {
    type Output = Self;

    /// Alias to [`StopwatchImpl::saturating_add`].
    #[inline]
    fn add(self, rhs: Duration) -> Self::Output {
        self.saturating_add(rhs)
    }
}

impl<I: Instant> ops::Sub<Duration> for StopwatchImpl<I> {
    type Output = Self;

    /// Alias to [`StopwatchImpl::saturating_sub`].
    #[inline]
    fn sub(self, rhs: Duration) -> Self::Output {
        self.saturating_sub(rhs)
    }
}

impl<I: Instant> ops::AddAssign<Duration> for StopwatchImpl<I> {
    #[inline]
    fn add_assign(&mut self, rhs: Duration) {
        *self = *self + rhs;
    }
}

impl<I: Instant> ops::SubAssign<Duration> for StopwatchImpl<I> {
    #[inline]
    fn sub_assign(&mut self, rhs: Duration) {
        *self = *self - rhs;
    }
}

impl<I: Instant> PartialEq for StopwatchImpl<I> {
    /// Tests for equality between `self` and `rhs`.
    ///
    /// Stopwatches are equal if whether they are running and their elapsed time
    /// are equal.
    fn eq(&self, rhs: &Self) -> bool {
        let mut self_ = *self;
        let mut rhs_ = *rhs;
        let self_err = self_.normalize_start();
        let rhs_err = rhs_.normalize_start();

        self_.is_running() == rhs_.is_running()
            && self_.elapsed == rhs_.elapsed
            && self_err == rhs_err
    }
}

impl<I: Instant> Hash for StopwatchImpl<I> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut self_ = *self;
        let err = self_.normalize_start();

        self_.is_running().hash(state);
        self_.elapsed.hash(state);
        err.hash(state);
    }
}
