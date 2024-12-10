// Copyright (c) 2024 INVAP, open@invap.com.ar
// SPDX-License-Identifier: AGPL-3.0-or-later OR Fundacion-Sadosky-Commercial

use std::time::{SystemTime, UNIX_EPOCH};

#[derive(PartialEq, Clone, Copy)]
pub enum BaseTime {
    Zero,
    Epoch,
}

pub struct Stopwatch {
    has_started: bool,
    is_paused: bool,
    zero: BaseTime,
    start_time: u64,   // Microseconds since base time
    drag_time: u64,    // Total paused time in microseconds
    pause_start: u64,  // Time when the stopwatch was paused
}

impl Stopwatch {
    pub const fn new() -> Self {
        Self {
            has_started: false,
            is_paused: false,
            zero: BaseTime::Zero,
            start_time: 0,
            drag_time: 0,
            pause_start: 0,
        }
    }

    pub fn start(&mut self, base_time: BaseTime) {
        if !self.has_started {
            self.zero = base_time;
            self.start_time = match base_time {
                BaseTime::Zero => Self::current_time_microseconds(),
                BaseTime::Epoch => 0,
            };
            self.has_started = true;
            self.drag_time = 0;
            self.is_paused = false;
            self.pause_start = 0;
        } else {
            panic!("Stopwatch already started");
        }
    }

    pub fn pause(&mut self) {
        if self.has_started {
            if !self.is_paused {
                self.is_paused = true;
                self.pause_start = Self::current_time_microseconds();
            } else {
                panic!("Stopwatch is already paused");
            }
        } else {
            panic!("Stopwatch has not been started");
        }
    }

    pub fn resume(&mut self) {
        if self.has_started {
            if self.is_paused {
                let pause_end = Self::current_time_microseconds();
                self.drag_time += pause_end - self.pause_start;
                self.is_paused = false;
                self.pause_start = 0;
            } else {
                panic!("Stopwatch is not in pause");
            }
        } else {
            panic!("Stopwatch has not been started");
        }
    }

    pub fn get_time(&self) -> u64 {
        if self.has_started {
            if self.is_paused {
                self.pause_start - self.start_time
            } else {
                let current_time = Self::current_time_microseconds();
                current_time - self.start_time - self.drag_time
            }
        } else {
            panic!("Stopwatch has not been started");
        }
    }

    fn current_time_microseconds() -> u64 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("SystemTime before UNIX EPOCH");
        now.as_secs() * 1_000_000 + now.subsec_micros() as u64
    }
}
