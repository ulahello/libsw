// libsw: stopwatch library
// copyright (C) 2022-2023 Ula Shipman <ula.hello@mailbox.org>
// licensed under MIT OR Apache-2.0

use libsw_core::Stopwatch as CoreSw;

use core::fmt;
use core::hash::{Hash, Hasher};
use core::ops;
use core::time::Duration;

use crate::{Error, Guard, Instant};

/// A stopwatch measures and accumulates elapsed time between starts and stops.
///
/// Stopwatches work with any type that implements [`Instant`].
#[derive(Clone, Copy)]
#[allow(clippy::module_name_repetitions)]
pub struct StopwatchImpl<I: Instant> {
    pub(crate) inner: CoreSw<I>,
}

impl<I: Instant> StopwatchImpl<I> {
    /// Returns a stopped stopwatch with zero elapsed time.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Sw;
    /// # use core::time::Duration;
    /// let sw = Sw::new();
    /// assert!(sw.is_stopped());
    /// assert_eq!(sw.elapsed(), Duration::ZERO);
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        Self::with_elapsed(Duration::ZERO)
    }

    /// Returns a running stopwatch initialized with zero elapsed time.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Sw;
    /// let sw = Sw::new_started();
    /// assert!(sw.is_running());
    /// ```
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
    /// # use libsw::Sw;
    /// # use core::time::Duration;
    /// # use std::time::Instant;
    /// let now = Instant::now();
    /// let sw_1 = Sw::new_started_at(now);
    /// let sw_2 = Sw::new_started_at(now);
    /// // they've both started at the same time
    /// assert_eq!(sw_1, sw_2);
    /// // (and had zero elapsed time when they started)
    /// assert_eq!(sw_1.elapsed_at(now), Duration::ZERO);
    /// ```
    #[must_use]
    pub const fn new_started_at(start: I) -> Self {
        Self::from_raw(Duration::ZERO, Some(start))
    }

    /// Returns a stopped stopwatch with the given elapsed time.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Sw;
    /// # use core::time::Duration;
    /// let sw = Sw::with_elapsed(Duration::from_secs(1));
    /// assert!(sw.is_stopped());
    /// assert_eq!(sw.elapsed(), Duration::from_secs(1));
    /// ```
    #[must_use]
    pub const fn with_elapsed(elapsed: Duration) -> Self {
        Self::from_raw(elapsed, None)
    }

    /// Returns a running stopwatch initialized with the given elapsed time.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Sw;
    /// # use core::time::Duration;
    /// let sw = Sw::with_elapsed_started(Duration::from_secs(1));
    /// assert!(sw.is_running());
    /// assert!(sw.elapsed() >= Duration::from_secs(1));
    /// ```
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
    /// # use libsw::Sw;
    /// # use core::time::Duration;
    /// let sw = Sw::from_raw(Duration::from_secs(1), None);
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
    /// # use libsw::Sw;
    /// # use core::time::Duration;
    /// # use std::time::Instant;
    /// let mut elapsed = Duration::from_secs(10);
    /// let mut start = Instant::now();
    /// let sw_1 = Sw::from_raw(elapsed, Some(start));
    /// let sw_2 = Sw::from_raw(
    ///     elapsed - Duration::from_secs(1),     // now `elapsed()` is 1s less
    ///     Some(start - Duration::from_secs(1)), // now with start pushed back, `elapsed()` is equal
    /// );
    ///
    /// // different components, but they are equal!
    /// assert_eq!(sw_1, sw_2);
    /// ```
    #[must_use]
    pub const fn from_raw(elapsed: Duration, start: Option<I>) -> Self {
        Self::from_core(CoreSw::from_raw(elapsed, start))
    }

    /// Constructs a `StopwatchImpl` from a [`libsw_core::Stopwatch`].
    pub const fn from_core(core_sw: CoreSw<I>) -> Self {
        Self { inner: core_sw }
    }

    /// Returns a [`libsw_core::Stopwatch`] with the same elapsed time and start.
    pub const fn to_core(self) -> CoreSw<I> {
        self.inner
    }

    /// Returns `true` if the stopwatch is running.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Sw;
    /// let sw = Sw::new_started();
    /// assert!(sw.is_running());
    /// ```
    #[must_use]
    pub const fn is_running(&self) -> bool {
        self.inner.is_running()
    }

    /// Returns `true` if the stopwatch is stopped.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Sw;
    /// let sw = Sw::new();
    /// assert!(sw.is_stopped());
    /// ```
    #[must_use]
    pub const fn is_stopped(&self) -> bool {
        self.inner.is_stopped()
    }

    /// Returns the total time elapsed. If overflow occurs, the elapsed time is
    /// saturated to [`Duration::MAX`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Sw;
    /// # use core::time::Duration;
    /// # use std::thread;
    /// # fn main() -> libsw::Result<()> {
    /// let sw = Sw::new_started();
    /// thread::sleep(Duration::from_millis(100));
    /// assert!(sw.elapsed() >= Duration::from_millis(100));
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn elapsed(&self) -> Duration {
        self.inner.elapsed()
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
    /// # use libsw::Sw;
    /// # use std::time::Instant;
    /// let sw_1 = Sw::new_started();
    /// let sw_2 = sw_1.clone();
    /// let anchor = Instant::now();
    /// assert!(sw_1.elapsed_at(anchor) == sw_2.elapsed_at(anchor));
    /// ```
    #[must_use]
    pub fn elapsed_at(&self, anchor: I) -> Duration {
        self.inner.elapsed_at(anchor)
    }

    /// Computes the total time elapsed. If overflow occurred, returns [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Sw;
    /// # use core::time::Duration;
    /// # use std::thread;
    /// # fn main() -> libsw::Result<()> {
    /// let mut sw = Sw::new_started();
    /// thread::sleep(Duration::from_millis(100));
    /// assert!(sw.checked_elapsed().unwrap() >= Duration::from_millis(100));
    /// sw += Duration::MAX;
    /// assert!(sw.checked_elapsed().is_none());
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn checked_elapsed(&self) -> Option<Duration> {
        self.inner.checked_elapsed()
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
        self.inner.checked_elapsed_at(anchor)
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
    /// # use libsw::Sw;
    /// # use core::time::Duration;
    /// # use std::thread;
    /// let mut sw = Sw::new();
    /// assert!(sw.start().is_ok());
    /// assert!(sw.start().is_err());
    ///
    /// let then = sw.elapsed();
    /// thread::sleep(Duration::from_millis(100));
    /// let now = sw.elapsed();
    /// assert!(then != now);
    /// ```
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
    /// # use libsw::Sw;
    /// # use core::time::Duration;
    /// # use std::thread;
    /// let mut sw = Sw::new_started();
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
    /// # use libsw::Sw;
    /// # use core::time::Duration;
    /// # use std::thread;
    /// # use std::time::Instant;
    /// # fn main() -> libsw::Result<()> {
    /// let mut sw_1 = Sw::new();
    /// let mut sw_2 = Sw::new();
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
        if self.is_stopped() {
            self.inner.start_at(anchor);
            Ok(())
        } else {
            Err(Error::SwStart)
        }
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
    ///   elapsed time.
    ///
    /// - Overflows of the new elapsed time are saturated to [`Duration::MAX`].
    ///   Use [`StopwatchImpl::checked_stop_at`] to explicitly check for overflow.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Sw;
    /// # use core::time::Duration;
    /// # use std::thread;
    /// # use std::time::Instant;
    /// # fn main() -> libsw::Result<()> {
    /// let mut sw_1 = Sw::new_started();
    /// let mut sw_2 = sw_1.clone();
    /// let stop = Instant::now();
    /// sw_1.stop_at(stop)?;
    /// sw_2.stop_at(stop)?;
    /// assert_eq!(sw_1, sw_2);
    /// # Ok(())
    /// # }
    /// ```
    pub fn stop_at(&mut self, anchor: I) -> crate::Result<()> {
        if self.is_running() {
            self.inner.stop_at(anchor);
            Ok(())
        } else {
            Err(Error::SwStop)
        }
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
    /// # use libsw::Sw;
    /// # use core::time::Duration;
    /// # fn main() -> libsw::Result<()> {
    /// let mut sw = Sw::new_started();
    /// assert!(sw.checked_stop()?.is_some());
    /// sw.set(Duration::MAX);
    /// sw.start()?;
    /// assert!(sw.checked_stop()?.is_none());
    /// # Ok(())
    /// # }
    /// ```
    pub fn checked_stop(&mut self) -> crate::Result<Option<()>> {
        self.checked_stop_at(I::now())
    }

    /* TODO: these checked fallible type signatures are silly yet consistent */
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
        if self.is_running() {
            let overflow: bool = !self.inner.checked_stop_at(anchor);
            let flag = if overflow { None } else { Some(()) };
            Ok(flag)
        } else {
            Err(Error::SwStop)
        }
    }

    /// Toggles whether the stopwatch is running or stopped.
    ///
    /// # Notes
    ///
    /// See [`stop`](Self::stop) for details about how overflow is handled.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Sw;
    /// let mut sw = Sw::new();
    /// sw.toggle();
    /// assert!(sw.is_running());
    /// sw.toggle();
    /// assert!(sw.is_stopped());
    /// ```
    pub fn toggle(&mut self) {
        self.toggle_at(I::now());
    }

    /// Toggles whether the stopwatch is running or stopped, as if the current
    /// time were `anchor`.
    ///
    /// # Notes
    ///
    /// See [`start_at`](Self::start_at) and [`stop_at`](Self::stop_at) for
    /// notes about the chronology of `anchor`, as well as what happens if
    /// overflow occurs.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Sw;
    /// # use std::time::Instant;
    /// let mut left = Sw::new();
    /// let mut right = Sw::new_started();
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
        self.inner.toggle_at(anchor);
    }

    /// Tries to toggle whether the stopwatch is running or stopped. If the new
    /// elapsed time overflows, returns [`None`] without mutating the stopwatch.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Sw;
    /// # use core::time::Duration;
    /// # use std::thread;
    /// let mut sw = Sw::with_elapsed_started(Duration::MAX);
    /// thread::sleep(Duration::from_millis(100));
    /// // whoops, new elapsed time can't be Duration::MAX + 100ms
    /// assert!(sw.checked_toggle().is_none());
    /// ```
    #[must_use]
    pub fn checked_toggle(&mut self) -> Option<()> {
        self.checked_toggle_at(I::now())
    }

    /// Tries to toggle whether the stopwatch is running or stopped, as if the
    /// current time were `anchor`. If the new elapsed time overflows, returns
    /// [`None`] without mutating the stopwatch.
    ///
    /// # Examples
    ///
    /// See the documentation for [`checked_toggle`](Self::checked_toggle) for a
    /// related example.
    #[must_use]
    pub fn checked_toggle_at(&mut self, anchor: I) -> Option<()> {
        if self.inner.checked_toggle_at(anchor) {
            Some(())
        } else {
            None
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
    /// # use libsw::Sw;
    /// # use core::time::Duration;
    /// let mut sw = Sw::with_elapsed_started(Duration::from_secs(1));
    /// sw.reset();
    /// assert_eq!(sw, Sw::new());
    /// ```
    pub fn reset(&mut self) {
        self.inner.reset();
    }

    /// Resets the elapsed time to zero without affecting whether the stopwatch
    /// is running.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Sw;
    /// # use core::time::Duration;
    /// # fn main() -> libsw::Result<()> {
    /// let mut sw = Sw::with_elapsed_started(Duration::from_secs(1));
    /// sw.reset_in_place();
    /// assert!(sw.is_running());
    /// // new elapsed time is close to zero
    /// assert!(sw.elapsed() < Duration::from_millis(1));
    ///
    /// sw.stop()?;
    /// sw.reset_in_place();
    /// assert_eq!(sw, Sw::new());
    /// # Ok(())
    /// # }
    /// ```
    pub fn reset_in_place(&mut self) {
        self.inner.reset_in_place();
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
    pub fn reset_in_place_at(&mut self, start: I) {
        self.inner.reset_in_place_at(start);
    }

    /// Stops and sets the total elapsed time to `new`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Sw;
    /// # use core::time::Duration;
    /// let mut sw = Sw::new();
    /// sw.set(Duration::from_secs(1));
    /// assert_eq!(sw.elapsed(), Duration::from_secs(1));
    /// ```
    pub fn set(&mut self, new: Duration) {
        self.inner.set(new);
    }

    /// Sets the total elapsed time to `new` without affecting whether the
    /// stopwatch is running.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Sw;
    /// # use core::time::Duration;
    /// # fn main() -> libsw::Result<()> {
    /// let mut sw = Sw::new();
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
    pub fn set_in_place(&mut self, new: Duration) {
        self.inner.set_in_place(new);
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
        self.inner.set_in_place_at(new, anchor);
    }

    /// Stops and sets the total elapsed time to `new`, returning the previous
    /// elapsed time.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Sw;
    /// # use core::time::Duration;
    /// let mut sw = Sw::with_elapsed(Duration::from_secs(3));
    /// let previous = sw.replace(Duration::from_secs(1));
    /// assert_eq!(previous, Duration::from_secs(3));
    /// assert_eq!(sw.elapsed(), Duration::from_secs(1));
    /// ```
    pub fn replace(&mut self, new: Duration) -> Duration {
        self.inner.replace(new)
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
        self.inner.replace_at(new, anchor)
    }

    /// Adds `dur` to the total elapsed time. If overflow occurred, the total
    /// elapsed time is set to [`Duration::MAX`].
    ///
    /// ```
    /// # use libsw::Sw;
    /// # use core::time::Duration;
    /// let mut sw = Sw::with_elapsed(Duration::from_secs(1));
    /// sw = sw.saturating_add(Duration::from_secs(1));
    /// assert_eq!(sw.elapsed(), Duration::from_secs(2));
    /// sw = sw.saturating_add(Duration::MAX);
    /// assert_eq!(sw.elapsed(), Duration::MAX);
    /// ```
    #[must_use]
    pub const fn saturating_add(mut self, dur: Duration) -> Self {
        self.inner = self.inner.saturating_add(dur);
        self
    }

    /// Subtracts `dur` from the total elapsed time. If underflow occurred, the
    /// total elapsed time is set to [`Duration::ZERO`].
    ///
    /// # Notes
    ///
    /// See the documentation for [`saturating_sub_at`](Self::saturating_sub_at)
    /// for notes about positive overflow.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Sw;
    /// # use core::time::Duration;
    /// let mut sw = Sw::with_elapsed(Duration::from_secs(1));
    /// sw = sw.saturating_sub(Duration::from_secs(1));
    /// assert_eq!(sw.elapsed(), Duration::ZERO);
    /// sw = sw.saturating_sub(Duration::from_secs(1));
    /// assert_eq!(sw.elapsed(), Duration::ZERO);
    /// ```
    #[must_use]
    pub fn saturating_sub(self, dur: Duration) -> Self {
        self.saturating_sub_at(dur, I::now())
    }

    /// Subtracts `dur` from the total elapsed time, as if the current time were
    /// `anchor`. If underflow occurred, the total elapsed time is set to
    /// [`Duration::ZERO`].
    ///
    /// # Notes
    ///
    /// - If the elapsed time is overflowing (as in, would exceed
    ///   [`Duration::MAX`]), the elapsed time is clamped to [`Duration::MAX`] and
    ///   `dur` is subtracted from that.
    ///
    /// - `anchor` saturates to the last instant the stopwatch was started.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Sw;
    /// # use core::time::Duration;
    /// # use std::time::Instant;
    /// # use std::thread;
    /// let mut sw = Sw::new_started();
    /// thread::sleep(Duration::from_millis(100));
    /// let mut now = Instant::now();
    /// sw = sw.saturating_sub_at(Duration::from_secs(1), now);
    /// assert_eq!(sw.elapsed_at(now), Duration::ZERO);
    /// ```
    #[must_use]
    pub fn saturating_sub_at(mut self, dur: Duration, anchor: I) -> Self {
        self.inner = self.inner.saturating_sub_at(dur, anchor);
        self
    }

    /// Adds `dur` to the total elapsed time. If overflow occurred, returns
    /// [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Sw;
    /// # use core::time::Duration;
    /// let mut sw = Sw::new();
    /// sw = sw.checked_add(Duration::from_secs(1)).unwrap();
    /// assert_eq!(sw.elapsed(), Duration::from_secs(1));
    /// assert_eq!(sw.checked_add(Duration::MAX), None);
    /// ```
    #[must_use]
    pub const fn checked_add(mut self, dur: Duration) -> Option<Self> {
        match self.inner.checked_add(dur) {
            Some(new) => {
                self.inner = new;
                Some(self)
            }
            None => None,
        }
    }

    /// Subtracts `dur` from the total elapsed time. If overflow occurred,
    /// returns [`None`].
    ///
    /// # Notes
    ///
    /// See the documentation for [`checked_sub_at`](Self::checked_sub_at) for
    /// notes about positive overflow.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Sw;
    /// # use core::time::Duration;
    /// let mut sw = Sw::new();
    /// assert_eq!(sw.checked_sub(Duration::from_secs(1)), None);
    /// sw += Duration::from_secs(1);
    /// assert_eq!(
    ///     sw.checked_sub(Duration::from_secs(1)),
    ///     Some(Sw::with_elapsed(Duration::ZERO)),
    /// );
    /// ```
    #[must_use]
    pub fn checked_sub(self, dur: Duration) -> Option<Self> {
        self.checked_sub_at(dur, I::now())
    }

    /// Subtracts `dur` from the total elapsed time, as if the current time were
    /// `anchor`. If overflow occurred, returns [`None`].
    ///
    /// # Notes
    ///
    /// - Overflow can also occur if the elapsed time is overflowing (as in, would
    ///   exceed [`Duration::MAX`]).
    ///
    /// - `anchor` saturates to the last instant the stopwatch was started.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Sw;
    /// # use core::time::Duration;
    /// # use std::time::Instant;
    /// # use std::thread;
    /// let mut sw = Sw::new_started();
    /// thread::sleep(Duration::from_millis(100));
    /// let now = Instant::now();
    /// // underflow yields `None`
    /// assert_eq!(sw.checked_sub_at(Duration::from_secs(1), now), None);
    ///
    /// // positive overflow yields `None`
    /// sw.set_in_place(Duration::MAX);
    /// assert_eq!(sw.checked_sub(Duration::ZERO), None);
    /// ```
    #[must_use]
    pub fn checked_sub_at(mut self, dur: Duration, anchor: I) -> Option<Self> {
        self.inner = self.inner.checked_sub_at(dur, anchor)?;
        Some(self)
    }
}

impl<I: Instant> From<StopwatchImpl<I>> for CoreSw<I> {
    fn from(val: StopwatchImpl<I>) -> Self {
        val.inner
    }
}

impl<I: Instant> From<CoreSw<I>> for StopwatchImpl<I> {
    fn from(core_sw: CoreSw<I>) -> Self {
        Self { inner: core_sw }
    }
}

impl<I: Instant> fmt::Debug for StopwatchImpl<I> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("StopwatchImpl")
            .field("elapsed", &self.inner.elapsed)
            .field("start", &self.inner.start)
            .finish()
    }
}

impl<I: Instant> Default for StopwatchImpl<I> {
    /// Returns the default stopwatch. Same as calling [`StopwatchImpl::new`].
    fn default() -> Self {
        Self::new()
    }
}

impl<I: Instant> ops::Add<Duration> for StopwatchImpl<I> {
    type Output = Self;

    /// Add `dur` to `self`.
    ///
    /// Currently this is an alias to [`StopwatchImpl::checked_add`], but that
    /// is not a stable guarentee. If you need a guarentee on the
    /// implementation, use the [checked](Self::checked_add) or
    /// [saturating](Self::checked_add) methods explicitly.
    ///
    /// # Panics
    ///
    /// Panics if overflow occurs.
    #[track_caller]
    fn add(self, dur: Duration) -> Self::Output {
        self.inner.add(dur).into()
    }
}

impl<I: Instant> ops::Sub<Duration> for StopwatchImpl<I> {
    type Output = Self;

    /// Subtract `dur` from `self`.
    ///
    /// Currently this is an alias to [`StopwatchImpl::checked_sub`], but that
    /// is not a stable guarentee. If you need a guarentee on the
    /// implementation, use the [checked](Self::checked_sub) or
    /// [saturating](Self::checked_sub) methods explicitly.
    ///
    /// # Panics
    ///
    /// Panics if overflow occurs.
    #[track_caller]
    fn sub(self, dur: Duration) -> Self::Output {
        self.inner.sub(dur).into()
    }
}

impl<I: Instant> ops::AddAssign<Duration> for StopwatchImpl<I> {
    #[track_caller]
    fn add_assign(&mut self, dur: Duration) {
        self.inner.add_assign(dur);
    }
}

impl<I: Instant> ops::SubAssign<Duration> for StopwatchImpl<I> {
    #[track_caller]
    fn sub_assign(&mut self, dur: Duration) {
        self.inner.sub_assign(dur);
    }
}

impl<I: Instant> PartialEq for StopwatchImpl<I> {
    /// Tests for equality between `self` and `rhs`.
    ///
    /// Stopwatches are equal if whether they are running and their elapsed time
    /// are equal.
    fn eq(&self, rhs: &Self) -> bool {
        self.inner.eq(&rhs.inner)
    }
}

impl<I: Instant> Eq for StopwatchImpl<I> {}

impl<I: Instant + Hash> Hash for StopwatchImpl<I> {
    /// Hashes `self` and `rhs`. These hashes are not dependent on the time of
    /// measurement, so they can be used to test equality.
    ///
    /// # Support
    ///
    /// `I` (the [`Instant`] type used by the stopwatch) must implement
    /// [`Hash`].
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }
}
