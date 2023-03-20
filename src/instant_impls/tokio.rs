// libsw: stopwatch library
// copyright (C) 2022-2023 Ula Shipman <ula.hello@mailbox.org>
// licensed under MIT OR Apache-2.0

use crate::Instant;

use core::time::Duration;

impl Instant for tokio::time::Instant {
    #[inline]
    fn now() -> Self {
        // NOTE: tokio::time::pause can freeze time
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
        self.saturating_duration_since(earlier)
    }
}
