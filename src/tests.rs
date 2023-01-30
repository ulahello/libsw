// libsw: stopwatch library (tests)
// copyright (C) 2022-2023 Ula Shipman <ula.hello@mailbox.org>
// licensed under MIT OR Apache-2.0

// TODO: re-organize tests

use crate::Error;

use core::hash::{Hash, Hasher};
use core::time::Duration;
use std::collections::hash_map::DefaultHasher;
use std::thread;

// TODO: manually changing these aliases if i want to test all supported
// `Instant` impls is annoying
use std::time::Instant;
type Stopwatch = crate::stopwatch::Stopwatch<Instant>;

const DELAY: Duration = Duration::from_millis(100);

#[test]
fn default() {
    let sw = Stopwatch::default();
    assert_eq!(sw, Stopwatch::new());
}

#[test]
fn is_running() {
    let mut sw = Stopwatch::new();
    assert!(!sw.is_running());

    sw.start().unwrap();
    assert!(sw.is_running());

    sw.stop().unwrap();
    assert!(!sw.is_running());
}

#[test]
fn is_stopped() {
    let mut sw = Stopwatch::new();
    assert!(sw.is_stopped());

    sw.start().unwrap();
    assert!(!sw.is_stopped());

    sw.stop().unwrap();
    assert!(sw.is_stopped());
}

#[test]
fn toggle() {
    let mut sw = Stopwatch::new();
    assert!(sw.is_stopped());

    sw.toggle();
    assert!(sw.is_running());

    sw.toggle();
    assert!(sw.is_stopped());
}

#[test]
fn reset() {
    let mut sw = Stopwatch::new_started();
    thread::sleep(DELAY);
    sw.stop().unwrap();
    sw.start().unwrap();
    sw.reset();
    assert_eq!(sw, Stopwatch::new())
}

#[test]
fn set() {
    let mut sw = Stopwatch::new_started();
    sw.set(DELAY);
    assert_eq!(sw, Stopwatch::with_elapsed(DELAY));
}

#[test]
fn set_in_place() -> crate::Result<()> {
    let mut sw = Stopwatch::new_started();
    sw.set_in_place(DELAY);
    assert!(sw.is_running());
    assert!(sw.elapsed() >= DELAY);

    thread::sleep(DELAY);

    sw.set_in_place(DELAY);
    assert!(sw.is_running());
    assert!(sw.elapsed() < DELAY * 2);

    Ok(())
}

#[test]
fn replace() {
    let mut sw = Stopwatch::with_elapsed_started(DELAY);
    let prev = sw.replace(DELAY * 2);

    assert!(sw.is_stopped());
    assert!(prev >= DELAY);
    assert_eq!(sw.elapsed(), DELAY * 2);
}

#[test]
fn add() {
    let mut sw = Stopwatch::new();

    sw += DELAY;
    sw.start().unwrap();
    sw += DELAY;
    sw.stop().unwrap();
    sw += DELAY;

    assert!(sw.elapsed() >= DELAY * 3);
}

#[test]
fn sub() {
    assert_eq!(
        Stopwatch::with_elapsed(DELAY * 3) - DELAY,
        Stopwatch::with_elapsed(DELAY * 2)
    );
}

#[test]
fn checked_add() {
    let mut sw = Stopwatch::new();

    sw = sw.checked_add(DELAY).unwrap();
    sw.start().unwrap();
    sw = sw.checked_add(DELAY).unwrap();
    sw.stop().unwrap();
    sw = sw.checked_add(DELAY).unwrap();

    assert!(sw.elapsed() >= DELAY * 3);
}

#[test]
fn checked_sub() {
    assert_eq!(
        Stopwatch::with_elapsed(DELAY * 3)
            .checked_sub(DELAY)
            .unwrap(),
        Stopwatch::with_elapsed(DELAY * 2)
    );
}

#[test]
fn checked_add_overflow() {
    assert_eq!(
        Stopwatch::new().checked_add(Duration::MAX).unwrap(),
        Stopwatch::with_elapsed(Duration::MAX),
    );
    assert_eq!(
        Stopwatch::with_elapsed(DELAY).checked_add(Duration::MAX),
        None,
    );
}

#[test]
fn checked_sub_overflow() {
    assert_eq!(
        Stopwatch::with_elapsed(Duration::MAX)
            .checked_sub(Duration::MAX)
            .unwrap(),
        Stopwatch::new(),
    );
    assert_eq!(Stopwatch::with_elapsed(DELAY).checked_sub(DELAY * 2), None);
}

#[test]
fn double_starts_stops_errs() {
    let mut sw = Stopwatch::new();

    assert_eq!(sw.start(), Ok(()));
    assert_eq!(sw.start(), Err(Error::SwStart));

    assert_eq!(sw.stop(), Ok(()));
    assert_eq!(sw.stop(), Err(Error::SwStop));
}

#[test]
fn sane_elapsed_while_stopped() {
    let mut sw = Stopwatch::new_started();
    thread::sleep(DELAY);
    sw.stop().unwrap();

    assert!(sw.elapsed() >= DELAY);
}

#[test]
fn sane_elapsed_while_running() {
    let sw = Stopwatch::new_started();
    thread::sleep(DELAY);

    assert!(sw.elapsed() >= DELAY);
}

#[test]
#[should_panic]
fn sync_before_sub_saturating() {
    let mut sw = Stopwatch::new_started();
    thread::sleep(DELAY);
    sw -= DELAY;
    assert!(sw.elapsed() >= DELAY);
}

#[test]
#[should_panic]
fn sync_before_sub_checked() {
    let mut sw = Stopwatch::new_started();
    thread::sleep(DELAY);
    sw = match sw.checked_sub(DELAY) {
        Some(new) => new,
        // test is expected to panic so return abnormally to indicate failure
        None => return,
    };
    assert!(sw.elapsed() >= DELAY);
}

#[test]
fn elapsed_at_saturates() {
    let sw = Stopwatch::with_elapsed_started(DELAY);
    assert_eq!(sw.elapsed_at(Instant::now() - (DELAY * 2)), DELAY);
}

#[test]
fn checked_elapsed_overflows() {
    assert!(Stopwatch::with_elapsed_started(Duration::MAX)
        .checked_elapsed()
        .is_none());
}

#[test]
fn start_in_future() {
    let mut sw = Stopwatch::new();
    sw.start_at(Instant::now() + (DELAY * 2)).unwrap();

    thread::sleep(DELAY);
    sw.stop().unwrap();
    assert_eq!(sw.elapsed(), Duration::ZERO);
}

#[test]
fn stop_before_last_start() {
    let mut sw = Stopwatch::with_elapsed(DELAY);
    let start = Instant::now();
    let old_elapsed = sw.elapsed();
    sw.start_at(start).unwrap();
    thread::sleep(DELAY);
    sw.stop_at(start - DELAY).unwrap();
    assert_eq!(old_elapsed, sw.elapsed());
}

#[test]
fn checked_stop_overflows() {
    let mut sw = Stopwatch::with_elapsed_started(Duration::MAX);
    thread::sleep(DELAY);
    assert!(sw.checked_elapsed().is_none());
    assert!(sw.checked_stop().unwrap().is_none());
    assert!(sw.is_running());
}

#[test]
fn checked_stop_stops() {
    let mut sw = Stopwatch::new_started();
    assert!(sw.is_running());
    sw.checked_stop().unwrap().unwrap();
    assert!(sw.is_stopped());
}

#[test]
fn eq_properties() {
    for [a, b, c] in mixed_stopwatches() {
        dbg!(a, b, c);

        // reflexive
        assert!(a == a);
        assert!(b == b);

        // symmetric
        assert_eq!(a == b, b == a);

        // transitive
        if (a == b) && (b == c) {
            assert_eq!(a, c);
        }
    }
}

#[test]
fn eq_running() {
    // whatever is compared shouldn't depend on the time of observation
    let sw_1 = Stopwatch::new_started();
    let sw_2 = sw_1.clone();
    assert_eq!(sw_1, sw_2);
}

#[test]
fn eq_correct() -> crate::Result<()> {
    assert_ne!(Stopwatch::new(), Stopwatch::new_started());
    assert_ne!(
        Stopwatch::with_elapsed(Duration::from_secs(1)),
        Stopwatch::with_elapsed(Duration::from_secs(2)),
    );

    let mut sw_1 = Stopwatch::new();
    let mut sw_2 = Stopwatch::new();
    let start = Instant::now();
    sw_1.start_at(start)?;
    sw_2.start_at(start)?;
    assert_eq!(sw_1, sw_2);

    Ok(())
}

#[test]
fn partial_eq() {
    for [a, b, _] in mixed_stopwatches() {
        assert_eq!(a == b, !(a != b));
    }
}

#[test]
fn hash_and_eq() {
    for [sw_1, sw_2, _] in mixed_stopwatches() {
        let mut hasher_1 = DefaultHasher::new();
        let mut hasher_2 = DefaultHasher::new();

        sw_1.hash(&mut hasher_1);
        sw_2.hash(&mut hasher_2);

        // > When implementing both Hash and Eq, it is important that the following property holds:
        // > k1 == k2 -> hash(k1) == hash(k2)
        assert_eq!(sw_1 == sw_2, hasher_1.finish() == hasher_2.finish());
    }
}

#[test]
fn hash_running() {
    let sw_1 = Stopwatch::new_started();
    let sw_2 = sw_1.clone();

    let mut hasher_1 = DefaultHasher::new();
    let mut hasher_2 = DefaultHasher::new();

    sw_1.hash(&mut hasher_1);
    sw_2.hash(&mut hasher_2);

    // whatever is hashed shouldn't depend on the time of observation
    assert_eq!(hasher_1.finish(), hasher_2.finish());
}

fn mixed_stopwatches() -> [[Stopwatch; 3]; 8] {
    let crafted_1;
    let crafted_2;
    {
        let mut elapsed = Duration::from_secs(10);
        let mut start = Instant::now();
        crafted_1 = Stopwatch::from_raw(elapsed, Some(start));

        elapsed -= Duration::from_secs(1);
        start = start.checked_sub(Duration::from_secs(1)).unwrap();
        crafted_2 = Stopwatch::from_raw(elapsed, Some(start));
    }
    assert_eq!(crafted_1, crafted_2);

    let started = Stopwatch::new_started();
    let started_elapsed = Stopwatch::with_elapsed_started(Duration::from_secs(1));
    [
        [Stopwatch::new(), Stopwatch::new(), Stopwatch::new()],
        [started, started, started],
        [started, Stopwatch::new(), Stopwatch::new()],
        [
            Stopwatch::with_elapsed(Duration::from_secs(1)),
            Stopwatch::with_elapsed(Duration::from_secs(1)),
            Stopwatch::with_elapsed(Duration::from_secs(1)),
        ],
        [started_elapsed, started_elapsed, started_elapsed],
        [
            started_elapsed,
            Stopwatch::with_elapsed(Duration::from_secs(1)),
            Stopwatch::with_elapsed(Duration::from_secs(1)),
        ],
        [
            Stopwatch::with_elapsed(Duration::from_secs(1)),
            Stopwatch::with_elapsed(Duration::from_secs(2)),
            Stopwatch::with_elapsed(Duration::from_secs(3)),
        ],
        [crafted_1, crafted_2, Stopwatch::default()],
    ]
}
