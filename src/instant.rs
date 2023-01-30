use core::fmt::Debug;
use core::time::Duration;

/// A trait outlining the behavior of a timekeeping type.
///
/// This trait allows `libsw` to be agnostic about timekeeping: any type which
/// implements `Instant` can be used within a [`Stopwatch`](crate::Stopwatch).
///
/// # Provided implementations
///
/// `libsw` provides `Instant` implementations for a number of timekeeping
/// types.
///
/// | Type                    | Feature flag     |
/// | ---                     | ---              |
/// | `std::time::Instant`    | `std_instant`    |
/// | `std::time::SystemTime` | `std_systemtime` |
/// | `tokio::time::Instant`  | `tokio`          |
pub trait Instant: Copy + Debug {
    /// Returns the current instant in time.
    fn now() -> Self
    where
        Self: Sized;

    /// Returns an instant ahead of `self` by the given [`Duration`] of time.
    ///
    /// Returns [`None`] if overflow occured, meaning the new instant was not
    /// representable with the underlying type.
    fn checked_add(&self, duration: Duration) -> Option<Self>
    where
        Self: Sized;

    /// Returns an instant previous to `self` by the given [`Duration`] of time.
    ///
    /// Returns [`None`] if overflow occured, meaning the new instant was not
    /// representable with the underlying type.
    fn checked_sub(&self, duration: Duration) -> Option<Self>
    where
        Self: Sized;

    /// Returns the [`Duration`] that has elapsed since `earlier`, returning
    /// [`Duration::ZERO`] if `earlier` is ahead of `self`.
    fn saturating_duration_since(&self, earlier: Self) -> Duration
    where
        Self: Sized;
}
