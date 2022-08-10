// libsw: stopwatch library
// copyright (C) 2022  Ula Shipman <ula.hello@mailbox.org>
// licensed under MIT OR GPL-3.0-or-later

use crate::Stopwatch;

/// A running, guarded, [`Stopwatch`]. When dropped, the `Stopwatch` will
/// automatically stop.
///
/// `Guard`s are returned by the `Stopwatch` methods
/// [`guard`](Stopwatch::guard) and [`guard_at`](Stopwatch::guard_at).
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

impl<'a> Drop for Guard<'a> {
    /// Releases the guard, calling [`stop`](Stopwatch::stop) on the guarded
    /// [`Stopwatch`].
    fn drop(&mut self) {
        debug_assert!(self.inner.is_running());
        let _ = self.inner.stop();
    }
}
