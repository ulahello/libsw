// libsw: stopwatch library
// copyright (C) 2022  Ula Shipman <ula.hello@mailbox.org>
// licensed under MIT OR GPL-3.0-or-later

use crate::Stopwatch;

use core::time::Duration;
use std::time::Instant;

/// A running, guarded, [`Stopwatch`]. When dropped, the `Stopwatch` will
/// automatically stop.
///
/// `Guard`s are returned by the `Stopwatch` methods [`guard`](Stopwatch::guard)
/// and [`guard_at`](Stopwatch::guard_at).
///
/// # Examples
///
/// ```
/// # use libsw::{Error, Stopwatch};
/// # use core::time::Duration;
/// # use std::thread;
/// # fn main() -> Result<(), Error> {
/// let mut sw = Stopwatch::new();
/// {
///     let _guard = sw.guard()?;
///     // stopwatch is now running and guarded!
///     thread::sleep(Duration::from_millis(100));
///     // guard dropped, stopwatch stopped
/// }
/// assert!(sw.is_stopped());
/// assert!(sw.elapsed() >= Duration::from_millis(100));
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct Guard<'a> {
    // invariant: sw must be running
    pub(crate) inner: &'a mut Stopwatch,
}

impl<'a> Guard<'a> {
    /// Returns the total time elapsed.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::{Error, Stopwatch};
    /// # use core::time::Duration;
    /// # use std::thread;
    /// # fn main() -> Result<(), Error> {
    /// let mut sw = Stopwatch::new();
    /// let guard = sw.guard()?;
    /// thread::sleep(Duration::from_millis(100));
    /// assert!(guard.elapsed() >= Duration::from_millis(100));
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub fn elapsed(&self) -> Duration {
        self.inner.elapsed()
    }

    /// Returns the total time elapsed, measured at the given [`Instant`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::{Error, Stopwatch};
    /// # use std::time::Instant;
    /// # fn main() -> Result<(), Error> {
    /// let mut sw_1 = Stopwatch::new();
    /// let mut sw_2 = Stopwatch::new();
    ///
    /// let start = Instant::now();
    /// let guard_1 = sw_1.guard_at(start)?;
    /// let guard_2 = sw_2.guard_at(start)?;
    ///
    /// let anchor = Instant::now();
    /// assert!(guard_1.elapsed_at(anchor) == guard_2.elapsed_at(anchor));
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub fn elapsed_at(&self, anchor: Instant) -> Duration {
        self.inner.elapsed_at(anchor)
    }
}

impl<'a> Drop for Guard<'a> {
    /// Releases the guard, calling [`stop`](Stopwatch::stop) on the guarded
    /// [`Stopwatch`].
    fn drop(&mut self) {
        debug_assert!(self.inner.is_running());
        let _ = self.inner.stop();
    }
}
