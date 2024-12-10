# A reporter API for Rust programs
This project provides a reporter API written in Rust, to be used in tandem with an instrumentation-based event reporting application like the [Runtime Reporter](https://github.com/invap/rt-reporter/ "The Runtime Reporter") for the runtime verification of Rust programs. 


## Installation
In this section we will review relevant aspects of how to setup this project for using it as a reporter API written in Rust for using the [Runtime Reporter](https://github.com/invap/rt-reporter/ "The Runtime Reporter") and the [Runtime Monitor](https://github.com/invap/rt-monitor/ "The Runtime Monitoring").

The implementation of the reporter library is distributed as source code to be used for instrumenting software artifacs. For obtaining it checkout the repository [rust-reporter-api](https://github.com/clpombo/rust-reporter-api/ "An implementation of a reporter API for instrumenting software artifacts, for the use of the Runtime Reporter and the Runtime Monitor").

### Base Rust language installation
[Rust](https://www.rust-lang.org) is installed and managed by the tool `rustup`. Detailed instructions to get it up and running can be found in [Install Rust](https://www.rust-lang.org/tools/install). There you can also find intructions for installing `rustup` through [different instalation methods](https://forge.rust-lang.org/index.html) in [Rust Forge](https://forge.rust-lang.org/). 

### Structure the project
The rust reporting library project is organized as follows:
```graphql
rust-reporter-api/
├── README_images/             # Images the read me file
│   └── class-diagram.png
├── src/                       # Source files
│   ├── rust-reporter-api.rs
│   ├── data_channel_defs.rs
│   └── stopwatch.rs
├── test/                      # Rust test files
│   ├── test_rust-reporter-api.rs
│   ├── test_data_channel_defs.rs
│   └── test_stopwatch.rs
├── COPYING                    # Licence of the project 
├── Cargo.lock 
├── Cargo.toml                 # Cargo configuration file.
└── README.md                  # Read me file of the project
```


## Implementation of the reporter API
[Figure 1](#class-diagram) shows that architectural view of the implementation of the Rust reporting API.

<figure id="class-diagram" style="text-align: center;">
  <img src="./README_images/class-diagram-new.png" width="600" alt="The architectural view of the implementation of the Rust reporting API.">
  <figcaption style="font-style: italic;"><b>Figure 1</b>: The architectural view of the implementation of the Rust reporting API.
  </figcaption>
</figure>

There, we find three main components:
1. the Rust reporting API (itself) [`rust-reporting-api.rs`](https://github.com/clpombo/rust-reporter-api/blob/main/src/rust-reporter-api.rs): it is the main component of the library as it implements the function [`report`](https://github.com/clpombo/rust-reporter-api/blob/main/src/rust-reporter-api.rs#L57), used by the software under test (SUT) for reporting events occurring along its execution (see Section [Rust reporting API component](#rust-reporting-api-component) for more details).
2. the data channel definitions [`data_channel_defs.rs`](https://github.com/clpombo/rust-reporter-api/blob/main/src/data_channel_defs.h): declares the constants and data structures used for managing the communication channel that connects the SUT and the reporting application (see Section [Data channel definitions](#data-channel-definitions) for more details).
3. the Stopwatch [`stopwatch.rs`](https://github.com/clpombo/rust-reporter-api/blob/main/src/stopwatch.c): implements a stopwatch for time-stamping events (see Section [Stopwatch component](#stopwatch-component) for more details).


### Rust reporting API component
The Rust reporting API itself implement declares three global variables:
- the package buffer and a pointer to the first empty place:
```rust
static mut BUFFER: [ReporterPkg; BUFFER_CAPACITY] = [ReporterPkg { time: 0, event_type: NoneEvent, event: [' ' as u8; MAX_EVENT_SIZE] }; BUFFER_CAPACITY];
static mut BUFFER_USED: usize = 0;
```
- a global stopwatch used for time-stamping packages:
```rust
pub static REPORTING_CLK: LazyLock<Mutex<Stopwatch>> = LazyLock::new(|| Mutex::new(Stopwatch::new()));
```
And implements two functions, from which only one is supposed to be used by the SUT, `report`:
```rust
fn pack_and_send(pkg: ReporterPkg) { ... }
pub fn report(event_type: EventType, event: &str) { ... }
```
The function `packtAndSend` implements buffer managing by appending packages until the buffer is full and sent through the communication channel.

The function `report` takes the event type and string passed as actual parameters and constructs the package, according to the package type, time-stamps it using the value of the global stopwatch `reporting_clk`, and process it using the function `packAndSend`.


### Data channel definitions
This component provides the definitions used for managing the communication channel between the SUT and the reporting application:
- maximum size of the buffer: 
```c
const BUFFER_SIZE: usize = 65536; // 64K (OS default) max string length.
```
- maximum size for a single event:
```rust
pub const MAX_EVENT_SIZE: usize = 1010 + 2; // 1010 bytes + 2 (padding for the null terminator)
```
This line defines the maximum size for the events reported (for example, in the [example application](https://github.com/clpombo/rt-monitor-rust-example-app/ "Runtime Monitor example application in Rust") accompanying this project, the longest event that is reported is 308 bytes long, plus 2 for the '\0\0' string terminator.
- maximum size for a package containing a single event:
```rust
const MAX_EVENT_PKG_SIZE: usize = MAX_EVENT_SIZE + 12; // MAX_EVENT_SIZE: event, 8: time, 4: event_type
```
- maximum size for a package containing a single event:
```rust
pub const BUFFER_CAPACITY: usize = BUFFER_SIZE / MAX_EVENT_PKG_SIZE;
```
- the event type as an enumeration: 
```rust
//classification of the different types of events
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
```
- the package sent across the communication channel consisting of a time-stamp, the event type and an event detailed as a fixed-size string:
``` rust
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ReporterPkg {
    pub time: u64, // Using u64 to store time in microseconds
    pub event_type: EventType,
    pub event: [u8; MAX_EVENT_SIZE],
}
```

### Stopwatch component
This component implements a simple stopwatch providing basic functionality for starting, pausing, resuming and getting the current elapsed time. 

The operation of the stopwatch relies on storing the global time, got from the system, at the moment the stopwatch is started (attribute `startTime` in the structure shown below), as a base time-stamp and then storing the accumulated time through which the stopwatch was paused  (attribute `dragTime` in the structure shown below).

The structure used to implement the stopwatch is:
```rust
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
```
The functions implementing the operation of the stopwatch are:
- `pub const fn new() -> Self {...}`: creates an instance of a stopwatch by initialising the attributes of the structure.
- `pub fn start(&mut self, base_time: BaseTime) {...}`: starts the stopwatch choosing the use of the basetime either in `ZERO`, for computing the time-stamps using `0` as the start time, or in `EPOCH`, for computing time-stamps based on epoch (January 1st., 1970, 0 hours, 0 minutes, 0 seconds), by storing the current time got from the system in the structure,
- `pub fn pause(&mut self) {...}`: pauses the stopwatch by setting the attribute `isPaused` to `true` in the structure of the stopwatch
- `pub fn resume(&mut self) {...}`: resumes the operation of the stopwatch by setting the attribute `isPaused` to `false` and adding the time through which the stopwatch was paused, to the attribute `dragTime` in the structure of the stopwatch, and
- `pub fn get_time(&self) -> u64 {...}`: returns the current time of the stopwatch by substracting the value of the attributes `startTime` and `dragTime` to the current time of the system.

### CPU time vs. Wall time
Depending on the project one might be interested in recording time with a different time frame, either chossing to time-stamp resorting to CPU time or to Wall time. The first one is obtained by pausing the stopwatch before the reporting actions, and then resuming it after them like in the example below, extracted from the function [`main`](https://github.com/clpombo/rt-monitor-rust-example-app/blob/main/src/main_instrumented.rs#L24) in the project [Example application for the Runtime Monitor](https://github.com/invap/rt-monitor-example-app/), code fragment from [Line 56](https://github.com/clpombo/rt-monitor-rust-example-app/blob/main/src/main_instrumented.rs#L61) to [Line 62](https://github.com/clpombo/rt-monitor-rust-example-app/blob/main/src/main_instrumented.rs#66):
```rust
addition = 0;
// [ INSTRUMENTACION: Variable assigned. ]
REPORTING_CLK.lock().expect("Failure to acquire lock on the reporting clock.").pause();
report(StateEvent, format!("variable_value_assigned,main_addition,{}", addition).as_str());
REPORTING_CLK.lock().expect("Failure to acquire lock on the reporting clock.").resume();
//
```
In this way, wa can tamper with the stopwatch in order to get time-stamps reasonably close to the running time of the process in order to avoid counting the time consumed while executing the code inserted for instrumentating the SUT for reporting.

Therefore, the invocation of the `pause` and `resume` operations is not mandatory but serves the purpose of time-stamping with marks closer to the CPU time; not using them provides a time-stamping strategy with marks closer to the wall time. Choosing one of these strategies strongly depend on the rationale under which time-stamps are to be interpreted. Needless to say that because of the use of a pipe for communicating the software under test and the event reporter is time-consuming, the use of the wall time heavily distorts the timeline by computing the time spent in reporting tasks as execution time of the SUT; an effect that might make the analysis of the timed constraints to fail erroneously stating that the implementation does not meet the desired properties. On the other hand, if time-stamps were to be used for coordinating events of procedures running on different machines, the wall time might be more adecuate than the CPU time.


## License

Copyright (c) 2024 Carlos Gustavo Lopez Pombo

This software is licensed under the Affero General Public License (AGPL) v3. If you use this software in a non-profit context, you can use the AGPL v3 license.

If you want to include this software in a paid product or service, you must negotiate a commercial license with us.

### Benefits of dual licensing:

It allows organizations to use the software for free for non-commercial purposes.

It provides a commercial option for organizations that want to include the software in a paid product or service.

It ensures that the software remains open source and available to everyone.

### Differences between the licenses:

The AGPL v3 license requires that any modifications or derivative works be released under the same license.

The commercial license does not require that modifications or derivative works be released under the same license, but it may include additional terms and conditions.

### How to access and use the licenses:

The AGPL v3 license is available for free on our website.

The commercial license can be obtained by contacting us at clpombo@gmail.com

### How to contribute to the free open source version:

Contributions to the free version can be submitted through GitHub.
You shall sign a DCO (Developer Certificate of Origin) for each submission from all the members of your organization. The OCD will be an option during submission at GitHub.

### How to purchase the proprietary version:

The proprietary version can be purchased by contacting us at clpombo@gmail.com
