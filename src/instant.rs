use core::fmt::Debug;
use core::time::Duration;

// TODO: this is infallible however some instants are not (`embedded_time` crate for example)
/// A trait outlining the behavior of a timekeeping type.
///
/// This trait allows `libsw` to be agnostic about timekeeping: any type which
/// implements `Instant` can be used within a
/// [`StopwatchImpl`](crate::StopwatchImpl).
///
/// # Provided implementations
///
/// `libsw` provides `Instant` implementations for a number of timekeeping
/// types.
///
/// | Type                    | Feature flag     |
/// |-------------------------|------------------|
/// | `std::time::Instant`    | `std_instant`    |
/// | `std::time::SystemTime` | `std_systemtime` |
/// | `tokio::time::Instant`  | `tokio`          |
///
/// If a timekeeping type you want to use isn't supported out of the box, please
/// consider [filing an issue](https://github.com/ulahello/libsw/issues) on
/// GitHub. If you already implemented `Instant` for it, consider sending a PR
/// upstream.
pub trait Instant: Copy + Debug + Sized {
    /// Returns the current instant in time.
    fn now() -> Self;

    /// Returns an instant ahead of `self` by the given [`Duration`] of time.
    ///
    /// Returns [`None`] if overflow occured, meaning the new instant was not
    /// representable with the underlying type.
    fn checked_add(&self, duration: Duration) -> Option<Self>;

    /// Returns an instant previous to `self` by the given [`Duration`] of time.
    ///
    /// Returns [`None`] if overflow occured, meaning the new instant was not
    /// representable with the underlying type.
    fn checked_sub(&self, duration: Duration) -> Option<Self>;

    /// Returns the [`Duration`] that has elapsed since `earlier`, returning
    /// [`Duration::ZERO`] if `earlier` is ahead of `self`.
    fn saturating_duration_since(&self, earlier: Self) -> Duration;
}
