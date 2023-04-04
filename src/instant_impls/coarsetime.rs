// libsw: stopwatch library
// copyright (C) 2022-2023 Ula Shipman <ula.hello@mailbox.org>
// licensed under MIT OR Apache-2.0

use core::time::Duration;

use crate::Instant;

impl Instant for coarsetime::Instant {
    #[inline]
    fn now() -> Self {
        Self::now()
    }

    fn checked_add(&self, duration: Duration) -> Option<Self> {
        let coarse_dur = coarsetime::Duration::from(duration);
        self.as_ticks()
            .checked_add(coarse_dur.as_ticks())
            .is_some()
            .then(|| *self + coarse_dur)
    }

    fn checked_sub(&self, duration: Duration) -> Option<Self> {
        let coarse_dur = coarsetime::Duration::from(duration);
        self.as_ticks()
            .checked_sub(coarse_dur.as_ticks())
            .is_some()
            .then(|| *self - coarse_dur)
    }

    #[inline]
    fn saturating_duration_since(&self, earlier: Self) -> Duration {
        self.duration_since(earlier).into()
    }
}
