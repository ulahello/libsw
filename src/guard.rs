// libsw: stopwatch library
// copyright (C) 2022-2023 Ula Shipman <ula.hello@mailbox.org>
// licensed under MIT OR Apache-2.0

/* TODO: this is very basic and that limits how useful it is.
# it'd be nice if:
## guards could overlap and mask/invert eachother

```text
sw: ....!!!...........!!..!!!......
guard1: ^ created           ^ dropped
guard2:    ^ created  ^ dropped
guard3:                 ^ created
                         ^ dropped
```

## you could give it a closure to run on the stopwatch when dropped
struct GuardFn { inner: &mut Stopwatch, callback: FnOnce(&mut Stopwatch) }
impl Drop for GuardFn {
    fn drop(&mut self) {
        (self.callback)(self.inner);
    }
}
*/

use crate::{Error, Instant, StopwatchImpl};

/// A running, guarded, [stopwatch](StopwatchImpl). When [dropped](Guard::drop),
/// the stopwatch will automatically stop.
///
/// `Guard`s are returned by the `StopwatchImpl` methods
/// [`guard`](StopwatchImpl::guard) and [`guard_at`](StopwatchImpl::guard_at).
///
/// # Examples
///
/// ```
/// # use libsw::Sw;
/// # use core::time::Duration;
/// # use std::thread;
/// # fn main() -> libsw::Result<()> {
/// let mut sw = Sw::new();
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
#[must_use = "if unused, the inner stopwatch will immediately stop again"]
#[derive(Debug, Hash)]
pub struct Guard<'sw, I: Instant> {
    // invariant: sw must be running
    inner: &'sw mut StopwatchImpl<I>,
}

impl<'sw, I: Instant> Guard<'sw, I> {
    /// Returns a `Guard` to a running [stopwatch](StopwatchImpl).
    ///
    /// # Errors
    ///
    /// If the stopwatch is stopped, returns [`GuardNew`](Error::GuardNew).
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::{Error, Guard, Sw};
    /// # fn main() -> libsw::Result<()> {
    /// use std::time::Instant;
    /// let mut sw = Sw::new();
    /// assert_eq!(Guard::new(&mut sw), Err(Error::GuardNew));
    ///
    /// sw.start()?;
    /// assert!(Guard::new(&mut sw).is_ok());
    /// assert!(sw.is_stopped());
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(sw: &'sw mut StopwatchImpl<I>) -> crate::Result<Self> {
        if sw.is_running() {
            Ok(Self { inner: sw })
        } else {
            Err(Error::GuardNew)
        }
    }

    /// Returns a reference to the inner [`StopwatchImpl`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use libsw::Sw;
    /// # use core::time::Duration;
    /// # use std::thread;
    /// # fn main() -> libsw::Result<()> {
    /// let mut sw = Sw::new();
    /// let guard = sw.guard()?;
    /// thread::sleep(Duration::from_millis(100));
    /// assert!(guard.inner().elapsed() >= Duration::from_millis(100));
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub const fn inner(&self) -> &StopwatchImpl<I> {
        self.inner
    }
}

impl<I: Instant> Drop for Guard<'_, I> {
    /// Releases the guard, calling [`stop`](StopwatchImpl::stop) on the guarded
    /// [stopwatch](StopwatchImpl).
    #[inline]
    fn drop(&mut self) {
        debug_assert!(self.inner.is_running());
        _ = self.inner.stop();
    }
}

impl<I: Instant> PartialEq for Guard<'_, I> {
    fn eq(&self, rhs: &Self) -> bool {
        self.inner() == rhs.inner()
    }
}

impl<I: Instant> Eq for Guard<'_, I> {}
