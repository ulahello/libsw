use crate::Instant;

use core::time::Duration;

impl Instant for time::Instant {
    #[inline]
    fn now() -> Self {
        Self::now()
    }

    #[inline]
    fn checked_add(&self, duration: Duration) -> Option<Self> {
        self.0.checked_add(duration).map(time::Instant)
    }

    #[inline]
    fn checked_sub(&self, duration: Duration) -> Option<Self> {
        self.0.checked_sub(duration).map(time::Instant)
    }

    #[inline]
    fn saturating_duration_since(&self, earlier: Self) -> Duration {
        self.0.saturating_duration_since(earlier.0)
    }
}
