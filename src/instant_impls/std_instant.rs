use crate::Instant;

use core::time::Duration;

// TODO: make this default I
impl Instant for std::time::Instant {
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
        self.saturating_duration_since(earlier)
    }
}
