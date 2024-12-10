// Copyright (c) 2024 INVAP, open@invap.com.ar
// SPDX-License-Identifier: AGPL-3.0-or-later OR Fundacion-Sadosky-Commercial

// Constants
const BUFFER_SIZE: usize = 65536; // 64K (OS default) max string length.
pub const MAX_EVENT_SIZE: usize = 1010 + 2; // 1010 bytes + 2 (padding for the null terminator)
const MAX_EVENT_PKG_SIZE: usize = MAX_EVENT_SIZE + 12;
pub const BUFFER_CAPACITY: usize = BUFFER_SIZE / MAX_EVENT_PKG_SIZE;

// Event Types Enum
#[repr(u32)] // Match C enum size
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EventType {
    TimedEvent = 0,
    StateEvent = 1,
    ProcessEvent = 2,
    ComponentEvent = 3,
    SelfLoggableComponentLogInitEvent = 4,
    SelfLoggableComponentEvent = 5,
    NoneEvent = 6
}

// Reporter package structure containing event and timestamp
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ReporterPkg {
    pub time: u64, // Using u64 to store time in microseconds
    pub event_type: EventType,
    pub event: [u8; MAX_EVENT_SIZE],
}

