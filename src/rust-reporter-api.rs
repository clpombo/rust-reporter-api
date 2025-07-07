// Copyright (c) 2024 Carlos Gustavo Lopez Pombo
// SPDX-License-Identifier: AGPL-3.0-or-later OR Carlos Gustavo Lopez Pombo-Commercial

use std::sync::{LazyLock, Mutex};


pub mod stopwatch;           // Import the stopwatch implementation
pub mod data_channel_defs;       // Import the data channel definitions

use data_channel_defs::{
    EventType,
    EventType::{
        TimedEvent,
        StateEvent,
        ProcessEvent,
        ComponentEvent,
        EndOfReportEvent
    }
};
use data_channel_defs::{
    BUFFER_CAPACITY,
    MAX_EVENT_SIZE
};
use stopwatch::Stopwatch;

use std::io::{self, Write};
use crate::data_channel_defs::EventType::NoneEvent;
use crate::data_channel_defs::ReporterPkg;

static mut BUFFER: [ReporterPkg; BUFFER_CAPACITY] = [ReporterPkg { time: 0, event_type: NoneEvent, event: [' ' as u8; MAX_EVENT_SIZE] }; BUFFER_CAPACITY]; // Global buffer
static mut BUFFER_USED: usize = 0;  // Tracks how many items are in the buffer

pub static REPORTING_CLK: LazyLock<Mutex<Stopwatch>> = LazyLock::new(|| Mutex::new(Stopwatch::new()));

fn pack_and_send(pkg: ReporterPkg, end_of_report: bool) {
    unsafe {
        if end_of_report {
            for _i in BUFFER_USED..BUFFER_CAPACITY-1 {
                BUFFER[BUFFER_USED] = pkg;
                BUFFER_USED += 1;
            }
        }
        BUFFER[BUFFER_USED] = pkg;
        BUFFER_USED += 1;
        if BUFFER_USED == BUFFER_CAPACITY {
            let buffer_slice = std::slice::from_raw_parts(
                BUFFER.as_ptr() as *const u8,
                BUFFER_CAPACITY * size_of::<ReporterPkg>(),
            );
            // Write all data and flush
            io::stdout()
                .write_all(buffer_slice)
                .expect("Failed to write to stdout");
            io::stdout().flush().expect("Failed to flush stdout");
            // Reset the buffer usage counter
            BUFFER_USED = 0;
        }
    }
}

pub fn report(event_type: EventType, event: &str) {
    let pkg: ReporterPkg;
    let time: u64 = REPORTING_CLK.lock().expect("Failure to acquire lock on the reporting clock.").get_time();
    let formatted_event: String = format!("{:<1$}", event, MAX_EVENT_SIZE - 2);
    let mut data: [u8; MAX_EVENT_SIZE] = [u8::try_from(' ').unwrap(); MAX_EVENT_SIZE];
    for (i, &byte) in formatted_event.as_bytes().iter().enumerate() {
        if i < data.len() {
            data[i] = byte; // Copy the bytes into the array
        }
    }
    data[MAX_EVENT_SIZE - 2] = 0u8;     // NULL terminator for string.
    data[MAX_EVENT_SIZE - 1] = 0u8;
    let end_of_report;
    match event_type {
        TimedEvent
        | StateEvent
        | ProcessEvent
        | ComponentEvent => {
            pkg = ReporterPkg {
                time,
                event_type,
                event: data
            };
            end_of_report = false;
        },
        EndOfReportEvent => {
            pkg = ReporterPkg {
                time,
                event_type,
                event: data
            };
            end_of_report = true;
        }
        NoneEvent => {
            panic!("None package type!")
        }
    };
    // Send the packaged event
    pack_and_send(pkg, end_of_report);
}
