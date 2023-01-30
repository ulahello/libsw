use crate::Instant;

use core::time::Duration;

impl Instant for std::time::SystemTime {
    fn now() -> Self
    where
        Self: Sized,
    {
        Self::now()
    }

    fn checked_add(&self, duration: Duration) -> Option<Self>
    where
        Self: Sized,
    {
        self.checked_add(duration)
    }

    fn checked_sub(&self, duration: Duration) -> Option<Self>
    where
        Self: Sized,
    {
        self.checked_sub(duration)
    }

    fn saturating_duration_since(&self, earlier: Self) -> Duration
    where
        Self: Sized,
    {
        // NOTE: SystemTime is not monotonic. see its documentation for
        // implications
        self.duration_since(earlier).unwrap_or(Duration::ZERO)
    }
}
