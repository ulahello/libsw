use core::time::Duration;

// TODO: make sure tests & crate-wide docs are correct before merging. its
// always funky when u change a fundamental assumption of the crate.

// TODO: consider loosening `Copy` requirement
/// A trait outlining the behavior of a timekeeping object.
///
/// This trait allows `libsw` to be agnostic about timekeeping: any type which
/// implements `Instant` can be used within a [`Stopwatch`](crate::Stopwatch).
///
/// # Provided implementations
///
/// `libsw` provides `Instant` implementations for a number of timekeeping
/// types.
///
/// | Implementor            | Feature flag |
/// | ---------------------- | ------------ |
/// | `std::time::Instant`   | `std`        |
/// | `tokio::time::Instant` | `tokio`      |
pub trait Instant: Copy {
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
