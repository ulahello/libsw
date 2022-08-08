// libsw: stopwatch library
// copyright (C) 2022  Ula Shipman <ula.hello@mailbox.org>
// licensed under MIT OR GPL-3.0-or-later

use crate::{Error, Stopwatch};

use core::time::Duration;
use std::thread;

const DELAY: Duration = Duration::from_millis(200);

#[test]
fn default() {
    let sw = Stopwatch::default();
    assert_eq!(sw.elapsed(), Duration::ZERO);
    assert!(sw.is_stopped());
}

#[test]
fn is_running() {
    let mut sw = Stopwatch::default();
    assert!(!sw.is_running());

    sw.start().unwrap();
    assert!(sw.is_running());

    sw.stop().unwrap();
    assert!(!sw.is_running());
}

#[test]
fn is_stopped() {
    let mut sw = Stopwatch::default();
    assert!(sw.is_stopped());

    sw.start().unwrap();
    assert!(!sw.is_stopped());

    sw.stop().unwrap();
    assert!(sw.is_stopped());
}

#[test]
fn toggle() {
    let mut sw = Stopwatch::default();
    assert!(sw.is_stopped());

    sw.toggle();
    assert!(sw.is_running());

    sw.toggle();
    assert!(sw.is_stopped());
}

#[test]
fn reset() {
    let mut sw = Stopwatch::default();

    sw.start().unwrap();
    thread::sleep(DELAY);

    sw.reset();

    assert!(sw.is_stopped());
    assert_eq!(sw.elapsed(), Duration::ZERO)
}

#[test]
fn set() {
    let mut sw = Stopwatch::default();

    sw.start().unwrap();
    sw.set(DELAY);

    assert!(sw.is_stopped());
    assert_eq!(sw.elapsed(), DELAY);
}

#[test]
fn add() {
    let mut sw = Stopwatch::default();

    sw += DELAY;

    sw.start().unwrap();
    sw += DELAY;
    assert!(sw.is_running());

    sw.stop().unwrap();
    sw += DELAY;
    assert!(sw.is_stopped());

    assert!(sw.elapsed() >= DELAY * 3);
}

#[test]
fn sub() {
    let mut sw = Stopwatch::new(DELAY * 3, false);

    sw -= DELAY;
    assert_eq!(sw.elapsed(), DELAY * 2);
    assert!(sw.is_stopped());
}

#[test]
fn double_starts_stops_errs() {
    let mut sw = Stopwatch::default();

    assert_eq!(sw.start(), Ok(()));
    assert_eq!(sw.start(), Err(Error::AlreadyStarted));

    assert_eq!(sw.stop(), Ok(()));
    assert_eq!(sw.stop(), Err(Error::AlreadyStopped));
}

#[test]
fn sane_elapsed_while_stopped() {
    let mut sw = Stopwatch::default();

    sw.start().unwrap();
    thread::sleep(DELAY);
    sw.stop().unwrap();

    assert!(sw.elapsed() >= DELAY);
}

#[test]
fn sane_elapsed_while_running() {
    let mut sw = Stopwatch::default();

    sw.start().unwrap();
    thread::sleep(DELAY);

    assert!(sw.elapsed() >= DELAY);
}
