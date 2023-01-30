use crate::Instant;

use core::time::Duration;

impl Instant for std::time::Instant {
    #[inline]
    fn now() -> Self
    where
        Self: Sized,
    {
        Self::now()
    }

    #[inline]
    fn checked_add(&self, duration: Duration) -> Option<Self>
    where
        Self: Sized,
    {
        self.checked_add(duration)
    }

    #[inline]
    fn checked_sub(&self, duration: Duration) -> Option<Self>
    where
        Self: Sized,
    {
        self.checked_sub(duration)
    }

    #[inline]
    fn saturating_duration_since(&self, earlier: Self) -> Duration
    where
        Self: Sized,
    {
        self.saturating_duration_since(earlier)
    }
}
