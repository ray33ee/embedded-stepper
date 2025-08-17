//! Built-in coil driver implementations for common steppers.
//!
//! Please note: These built in drivers do not check or propagate errors from
//! [set_low](`embedded_hal::digital::OutputPin::set_low`) or
//! [set_high](`embedded_hal::digital::OutputPin::set_high`). This may be added in the future if there
//! is a demand, but for now users can implement their own drivers and error handling.
//!
//! - [`two_wire::StepperMotor2`]: simple 2-pin wave drive
//! - [`four_wire::StepperMotor4`]: classic 4-wire sequence
//! - [`five_wire::StepperMotor5`]: 5-wire (often unipolar common + 4 coils)


mod two_wire;
mod four_wire;
mod five_wire;

pub use two_wire::StepperMotor2;
pub use four_wire::StepperMotor4;
pub use five_wire::StepperMotor5;
