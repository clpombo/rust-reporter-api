// Copyright (c) 2024 Carlos Gustavo Lopez Pombo
// SPDX-License-Identifier: AGPL-3.0-or-later OR Carlos Gustavo Lopez Pombo-Commercial

use rust_reporter_api::data_channel_defs::{
    EventType::{
        TimedEvent,
        StateEvent,
        ProcessEvent,
        ComponentEvent,
        SelfLoggableComponentLogInitEvent,
        SelfLoggableComponentEvent
    },
    ReporterPkg,
    MAX_EVENT_SIZE
};

#[test]
fn test_reporter_timed_event_pkg_build() {
    let reporter_pkg = ReporterPkg { time: 0, event_type: TimedEvent, event: [' ' as u8; MAX_EVENT_SIZE] };
    assert!(reporter_pkg.time == 0 && reporter_pkg.event_type == TimedEvent && reporter_pkg.event == [' ' as u8; MAX_EVENT_SIZE]);
}

#[test]
fn test_reporter_state_event_pkg_build() {
    let reporter_pkg = ReporterPkg { time: 0, event_type: StateEvent, event: [' ' as u8; MAX_EVENT_SIZE] };
    assert!(reporter_pkg.time == 0 && reporter_pkg.event_type == StateEvent && reporter_pkg.event == [' ' as u8; MAX_EVENT_SIZE]);
}

#[test]
fn test_reporter_process_event_pkg_build() {
    let reporter_pkg = ReporterPkg { time: 0, event_type: ProcessEvent, event: [' ' as u8; MAX_EVENT_SIZE] };
    assert!(reporter_pkg.time == 0 && reporter_pkg.event_type == ProcessEvent && reporter_pkg.event == [' ' as u8; MAX_EVENT_SIZE]);
}

#[test]
fn test_reporter_component_event_pkg_build() {
    let reporter_pkg = ReporterPkg { time: 0, event_type: ComponentEvent, event: [' ' as u8; MAX_EVENT_SIZE] };
    assert!(reporter_pkg.time == 0 && reporter_pkg.event_type == ComponentEvent && reporter_pkg.event == [' ' as u8; MAX_EVENT_SIZE]);
}

#[test]
fn test_reporter_self_loggable_component_log_init_event_pkg_build() {
    let reporter_pkg = ReporterPkg { time: 0, event_type: SelfLoggableComponentLogInitEvent, event: [' ' as u8; MAX_EVENT_SIZE] };
    assert!(reporter_pkg.time == 0 && reporter_pkg.event_type == SelfLoggableComponentLogInitEvent && reporter_pkg.event == [' ' as u8; MAX_EVENT_SIZE]);
}

#[test]
fn test_reporter_self_loggable_component_event_pkg_build() {
    let reporter_pkg = ReporterPkg { time: 0, event_type: SelfLoggableComponentEvent, event: [' ' as u8; MAX_EVENT_SIZE] };
    assert!(reporter_pkg.time == 0 && reporter_pkg.event_type == SelfLoggableComponentEvent && reporter_pkg.event == [' ' as u8; MAX_EVENT_SIZE]);
}
