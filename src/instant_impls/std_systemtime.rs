use crate::Instant;

use core::time::Duration;

impl Instant for std::time::SystemTime {
    #[inline]
    fn now() -> Self {
        Self::now()
    }

    #[inline]
    fn checked_add(&self, duration: Duration) -> Option<Self> {
        self.checked_add(duration)
    }

    #[inline]
    fn checked_sub(&self, duration: Duration) -> Option<Self> {
        self.checked_sub(duration)
    }

    #[inline]
    fn saturating_duration_since(&self, earlier: Self) -> Duration {
        // NOTE: SystemTime is not monotonic. see its documentation for
        // implications
        self.duration_since(earlier).unwrap_or(Duration::ZERO)
    }
}
