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
//! ```no_run
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