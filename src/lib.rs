#![no_std]

//! # embedded_stepper
//!
//! A tiny **`no_std`** stepper motor helper built on top of
//! [`embedded-hal`](https://docs.rs/embedded-hal/latest/embedded_hal/).
//!
//! This library is a Rust port of the Arduino stepper library. More information can be found
//! in the official Arduino [doc](https://docs.arduino.cc/libraries/stepper/) and the
//! [header](https://github.com/arduino-libraries/Stepper/blob/master/src/Stepper.h) and
//! [source](https://github.com/arduino-libraries/Stepper/blob/master/src/Stepper.cpp).
//!
//! ## Design
//! - You supply a _coil driver_ that implements [StepperMotor](`driver::StepperMotor`)
//! - The high-level [Stepper](`stepper::Stepper`) performs stepping exposing the same functions as Arduino stepper.
//! - Concrete coil drivers for common wirings live under [`motors`].
//!
//! ## Quick start
//!
//! The following example works with the esp32-c3 development board:
//!
//! ```no_run
//! //Setup clock and peripherals
//! let config = esp_hal::Config::default().with_cpu_clock(CpuClock::default());
//! let peripherals = esp_hal::init(config);
//!
//! //Get the 4 GPIO pins, change these values based on your wiring
//! let p1 = peripherals.GPIO0;
//! let p2 = peripherals.GPIO1;
//! let p3 = peripherals.GPIO2;
//! let p4 = peripherals.GPIO3;
//!
//! //Set up the 4 pins as output, low, no pull and Push/Pull (default output config)
//! let op1 = Output::new(p1, Level::Low, OutputConfig::default());
//! let op2 = Output::new(p2, Level::Low, OutputConfig::default());
//! let op3 = Output::new(p3, Level::Low, OutputConfig::default());
//! let op4 = Output::new(p4, Level::Low, OutputConfig::default());
//!
//! //Adjust these values to fit your stepper motor. See the Arduino stepper library for more info
//! let speed = 625;
//! let steps_per_rev = 48;
//!
//! //Instantiate the motor with the 4 output pins
//! let mut motor = create_stepper_4pin(op1, op2, op3, op4, Delay::new(), steps_per_rev);
//!
//! motor.set_speed(speed);
//!
//! //Turn motor
//! let _ = motor.step(steps_per_rev as i32 * 10);
//!
//! //Reverse motor
//! let _ = motor.step(steps_per_rev as i32 * -10);
//!
//! //Deenergise coils after use
//! let _ = motor.deenergise();
//! ```
pub mod driver;
pub mod stepper;
pub mod motors;

mod builders;

/// High-level controller alias for a **2-pin** motor + `DelayNs`.
pub type Stepper2<P1, P2, D> = stepper::Stepper<motors::StepperMotor2<P1, P2>, D>;

/// High-level controller alias for a **4-pin** motor + `DelayNs`.
pub type Stepper4<P1, P2, P3, P4, D> = stepper::Stepper<motors::StepperMotor4<P1, P2, P3, P4>, D>;

/// High-level controller alias for a **5-pin** motor + `DelayNs`.
pub type Stepper5<P1, P2, P3, P4, P5, D> = stepper::Stepper<motors::StepperMotor5<P1, P2, P3, P4, P5>, D>;

/// Factory: build a [`Stepper2`] from two pins and a delay.
pub use builders::create_stepper_2pin;

/// Factory: build a [`Stepper4`] from four pins and a delay.
pub use builders::create_stepper_4pin;

/// Factory: build a [`Stepper5`] from five pins and a delay.
pub use builders::create_stepper_5pin;