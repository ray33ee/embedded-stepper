
//! Low-level **coil driver** trait used by the high-level [Stepper](`crate::stepper::Stepper`).
//!
//! Implement this for your motor wiring to describe how each `this_step`
//! energizes pins. Errors bubble up from the underlying GPIO pins via
//! the associated `Error` type.
//!
//! Equivalent to the Arduino `stepMotor` function.

/// Generic stepper “coil driver” interface (no_std, embedded-hal compatible).
pub trait StepperMotor {
    type Error;

    /// Drive to a specific step (implementation decides sequence).
    ///
    /// Must take a step number, and use this to determine the coil sequence and energise accordingly
    ///
    /// Equivalent to the Arduino `stepMotor` function.
    fn step(&mut self, this_step: u32) -> Result<(), Self::Error>;

    /// De-energize all coils.
    ///
    /// Implementors should set all coil pins to `low`.
    ///
    /// Arduino stepper does not have this function, but it may be useful for drivers with no enable-disable
    fn clear(&mut self) -> Result<(), Self::Error>;
}
