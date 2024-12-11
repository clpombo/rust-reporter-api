// Copyright (c) 2024 Carlos Gustavo Lopez Pombo
// SPDX-License-Identifier: AGPL-3.0-or-later OR Carlos Gustavo Lopez Pombo-Commercial

use std::time::Duration;
use rust_reporter_api::data_channel_defs::{
    EventType::{
        TimedEvent,
        StateEvent,
        ProcessEvent,
        ComponentEvent,
        SelfLoggableComponentLogInitEvent,
        SelfLoggableComponentEvent
    }
};

use rust_reporter_api::{report, REPORTING_CLK};
use rust_reporter_api::stopwatch::BaseTime;

fn duration_to_microseconds(duration: Duration) -> u64 {
    duration.as_secs() * 1_000_000 + u64::from(duration.subsec_micros())
}

#[test]
fn test_reporter() {
    REPORTING_CLK.lock().expect("Failure to acquire lock on the reporting clock.").start(BaseTime::Epoch);
    let start_time = REPORTING_CLK.lock().expect("Failure to acquire lock on the reporting clock.").get_time();
    let mut time: u64;
    time = REPORTING_CLK.lock().expect("Failure to acquire lock on the reporting clock.").get_time() - start_time;
    assert!(time < duration_to_microseconds(Duration::from_millis(100)));       // Should be a bit higher than 100.
    REPORTING_CLK.lock().expect("Failure to acquire lock on the reporting clock.").pause();
    report(TimedEvent, "Here is a timed event!");
    REPORTING_CLK.lock().expect("Failure to acquire lock on the reporting clock.").resume();
    time = REPORTING_CLK.lock().expect("Failure to acquire lock on the reporting clock.").get_time() - start_time;
    assert!(time < duration_to_microseconds(Duration::from_millis(100)));       // Should be a bit higher than 100.
    REPORTING_CLK.lock().expect("Failure to acquire lock on the reporting clock.").pause();
    report(StateEvent, "Here is a state event!");
    REPORTING_CLK.lock().expect("Failure to acquire lock on the reporting clock.").resume();
    time = REPORTING_CLK.lock().expect("Failure to acquire lock on the reporting clock.").get_time() - start_time;
    assert!(time < duration_to_microseconds(Duration::from_millis(100)));       // Should be a bit higher than 100.
    REPORTING_CLK.lock().expect("Failure to acquire lock on the reporting clock.").pause();
    report(ProcessEvent, "Here is a process event!");
    REPORTING_CLK.lock().expect("Failure to acquire lock on the reporting clock.").resume();
    time = REPORTING_CLK.lock().expect("Failure to acquire lock on the reporting clock.").get_time() - start_time;
    assert!(time < duration_to_microseconds(Duration::from_millis(100)));       // Should be a bit higher than 100.
    REPORTING_CLK.lock().expect("Failure to acquire lock on the reporting clock.").pause();
    report(ComponentEvent, "Here is a component event!");
    REPORTING_CLK.lock().expect("Failure to acquire lock on the reporting clock.").resume();
    time = REPORTING_CLK.lock().expect("Failure to acquire lock on the reporting clock.").get_time() - start_time;
    assert!(time < duration_to_microseconds(Duration::from_millis(100)));       // Should be a bit higher than 100.
    REPORTING_CLK.lock().expect("Failure to acquire lock on the reporting clock.").pause();
    report(SelfLoggableComponentLogInitEvent, "Here is a self loggable component log init event!");
    REPORTING_CLK.lock().expect("Failure to acquire lock on the reporting clock.").resume();
    time = REPORTING_CLK.lock().expect("Failure to acquire lock on the reporting clock.").get_time() - start_time;
    assert!(time < duration_to_microseconds(Duration::from_millis(100)));       // Should be a bit higher than 100.
    REPORTING_CLK.lock().expect("Failure to acquire lock on the reporting clock.").pause();
    report(SelfLoggableComponentEvent, "Here is a self loggable component  event!");
    REPORTING_CLK.lock().expect("Failure to acquire lock on the reporting clock.").resume();
    time = REPORTING_CLK.lock().expect("Failure to acquire lock on the reporting clock.").get_time() - start_time;
    assert!(time < duration_to_microseconds(Duration::from_millis(100)));       // Should be a bit higher than 100.
}