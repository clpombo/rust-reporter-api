// Copyright (c) 2024 INVAP, open@invap.com.ar
// SPDX-License-Identifier: AGPL-3.0-or-later OR Fundacion-Sadosky-Commercial

use rust_reporter_api::stopwatch::{
    Stopwatch,
    BaseTime
};

use std::time::Duration;

fn duration_to_microseconds(duration: Duration) -> u64 {
    duration.as_secs() * 1_000_000 + u64::from(duration.subsec_micros())
}

#[test]
fn test_stopwatch_start() {
    let mut clock: Stopwatch = Stopwatch::new();

    clock.start(BaseTime::Zero);
    std::thread::sleep(Duration::from_millis(100));
    clock.pause();
    let elapsed: u64 = clock.get_time();
    assert!(elapsed >= duration_to_microseconds(Duration::from_millis(100)));
    std::thread::sleep(Duration::from_millis(100));
    assert_eq!(elapsed, clock.get_time());
}

#[test]
fn test_stopwatch_pause() {
    let mut clock: Stopwatch = Stopwatch::new();

    clock.start(BaseTime::Zero);
    std::thread::sleep(Duration::from_millis(100));
    clock.pause();
    let mut elapsed: u64 = clock.get_time();
    assert!(elapsed >= duration_to_microseconds(Duration::from_millis(100)));
    std::thread::sleep(Duration::from_millis(100));
    elapsed += clock.get_time();
    assert!(elapsed >= duration_to_microseconds(Duration::from_millis(100)));
}

#[test]
fn test_stopwatch_pause_resume() {
    let mut clock: Stopwatch = Stopwatch::new();

    clock.start(BaseTime::Zero);
    std::thread::sleep(Duration::from_millis(100));
    clock.pause();
    let mut elapsed: u64 = clock.get_time();
    assert!(elapsed >= duration_to_microseconds(Duration::from_millis(100)));
    clock.resume();
    std::thread::sleep(Duration::from_millis(100));
    elapsed += clock.get_time();
    assert!(elapsed >= 2 * duration_to_microseconds(Duration::from_millis(100)));
    clock.pause();
    std::thread::sleep(Duration::from_millis(100));
    elapsed += clock.get_time();
    assert!(elapsed >= 2 * duration_to_microseconds(Duration::from_millis(100)));
}