// libsw: stopwatch library
// copyright (C) 2022  Ula Shipman <ula.hello@mailbox.org>
// licensed under MIT OR GPL-3.0-or-later

use crate::{Error, Stopwatch};

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
/// # use libsw::Stopwatch;
/// # use core::time::Duration;
/// # use std::thread;
/// # fn main() -> libsw::Result<()> {
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
#[must_use]
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Guard<'a> {
    // invariant: sw must be running
    inner: &'a mut Stopwatch,
}

impl<'a> Guard<'a> {
    /// Returns a `Guard` to a running [`Stopwatch`].
    ///
    /// # Errors
    ///
    /// If the stopwatch is stopped, returns [`GuardNew`](Error::GuardNew).
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::{Error, Guard, Stopwatch};
    /// # fn main() -> libsw::Result<()> {
    /// let mut sw = Stopwatch::new();
    /// assert_eq!(Guard::new(&mut sw), Err(Error::GuardNew));
    ///
    /// sw.start()?;
    /// assert!(Guard::new(&mut sw).is_ok());
    /// assert!(sw.is_stopped());
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn new(sw: &'a mut Stopwatch) -> crate::Result<Self> {
        sw.is_running()
            .then(|| Self { inner: sw })
            .ok_or(Error::GuardNew)
    }

    /// Returns the total time elapsed of the guarded [`Stopwatch`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Stopwatch;
    /// # use core::time::Duration;
    /// # use std::thread;
    /// # fn main() -> libsw::Result<()> {
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

    /// Returns the total time elapsed of the guarded [`Stopwatch`], measured at
    /// the given [`Instant`].
    ///
    /// # Notes
    ///
    /// This calls [`Stopwatch::elapsed_at`] on the guarded stopwatch, so you
    /// can expect the same behavior.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Stopwatch;
    /// # use std::time::Instant;
    /// # fn main() -> libsw::Result<()> {
    /// let mut sw_1 = Stopwatch::new();
    /// let mut sw_2 = Stopwatch::new();
    ///
    /// let start = Instant::now();
    /// let guard_1 = sw_1.guard_at(start)?;
    /// let guard_2 = sw_2.guard_at(start)?;
    ///
    /// assert!(guard_1 == guard_2);
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
    #[inline]
    fn drop(&mut self) {
        debug_assert!(self.inner.is_running());
        let _ = self.inner.stop();
    }
}
