# embedded-stepper

[![Crates.io](https://img.shields.io/crates/v/random_access_rng)](https://crates.io/crates/embedded-stepper)
[![Documentation](https://docs.rs/random_access_rng/badge.svg)](https://docs.rs/embedded-stepper)
[![Version](https://img.shields.io/badge/version-0.1.0-blue.svg)](https://github.com/ray33ee/embedded-stepper)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

> **no_std stepper motor library for Rust, built on [`embedded-hal`](https://docs.rs/embedded-hal) — and a *port of the classic Arduino `Stepper`* interface/behavior.**

This crate provides a tiny, portable stepper motor controller for bare-metal targets. It reuses the familiar Arduino `Stepper` API concepts (steps per revolution, `setSpeed`, `step`) while embracing Rust’s generics and the [`embedded-hal`] traits for GPIO and timing.

## Highlights

- **Port of Arduino `Stepper`**: same model and sequencing style, now in Rust.
- **`no_std`**: suitable for MCUs; no heap, no OS.
- **`embedded-hal` 1.0** compatible:
  - Pin control via `embedded_hal::digital::OutputPin`
  - Timing via `embedded_hal::delay::DelayNs`
- **Common wirings included**: 2‑pin, 4‑pin, and 5‑pin coil drivers
- **Driver trait**: users can implement drivers for other types of motors
- **Ergonomic builders & type aliases** for shorter type signatures.

## Quick Start

The following example works with the esp32-c3 development board:

```rust
#![no_std]
//Setup clock and peripherals
let config = esp_hal::Config::default().with_cpu_clock(CpuClock::default());
let peripherals = esp_hal::init(config);

//Get the 4 GPIO pins, change these values based on your wiring
let p1 = peripherals.GPIO0;
let p2 = peripherals.GPIO1;
let p3 = peripherals.GPIO2;
let p4 = peripherals.GPIO3;

//Setup the 4 pins as output, low, no pull and Push/Pull (default output config)
let op1 = Output::new(p1, Level::Low, OutputConfig::default());
let op2 = Output::new(p2, Level::Low, OutputConfig::default());
let op3 = Output::new(p3, Level::Low, OutputConfig::default());
let op4 = Output::new(p4, Level::Low, OutputConfig::default());

//Adjust these values to fit your stepper motor. See the Arduino stepper library for more info
let speed = 625;
let steps_per_rev = 48;

//Instantiate the motor with the 4 output pins
let mut motor = create_stepper_4pin(op1, op2, op3, op4, Delay::new(), steps_per_rev);

motor.set_speed(speed);

//Turn motor
let _ = motor.step(steps_per_rev as i32 * 10);

//Reverse motor
let _ = motor.step(steps_per_rev as i32 * -10);

//Deenergise coils after use
let _ = motor.deenergise();
```

## Design Notes

- **Arduino parity**: The public API mirrors Arduino’s `Stepper` where it makes sense (e.g., `set_speed`, `step`, steps per revolution). Internally, the controller is generic and decoupled.
- **Blocking by design**: `Stepper::step` is *blocking*. For tight real-time loops, consider calling it from a cooperative scheduler or extend the crate with a non-blocking `tick()` style API.
- **Wiring/Sequences**: 2/4/5 pin drivers implement simple 4-state sequences. Adjust sequences if your hardware requires half-stepping or microstepping—this crate provides a foundation rather than heavy abstraction.

## Current Limitations (and Roadmap)

- **Error handling**: The built-in drivers currently use `type Error = ()` and ignore pin I/O errors (they `let _ = pin.set_*()`).  
  **Planned**: bind each driver’s `Error` to the underlying pin error (`type Error = <P1 as OutputPin>::Error`) and propagate with `?` for full `embedded-hal` compliance.
- **Non-blocking control**: Provide an optional non-blocking controller (`tick()`), plus acceleration/deceleration profiles.

## Installing

Add to your `Cargo.toml`:

```toml
[dependencies]
embedded-stepper = { git = "https://github.com/ray33ee/embedded-stepper", default-features = false }
embedded-hal = { version = "1.0", default-features = false }
```

This crate is `no_std` by default; keep `default-features = false`.

## License

MIT © 2025 — see [LICENSE](./LICENSE).

## Acknowledgements

This work intentionally ports the behavior and ergonomics of the **Arduino `Stepper`** library to Rust, while integrating with the `embedded-hal` ecosystem.
