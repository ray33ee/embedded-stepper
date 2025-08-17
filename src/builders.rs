#![allow(clippy::needless_pass_by_value)]

use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;

use crate::{
    stepper::Stepper,
    motors::{StepperMotor2, StepperMotor4, StepperMotor5},
};

/// Build a 2-pin stepper controller.
///
/// - `number_of_steps`: steps per revolution (or per full cycle of your sequence)
pub fn create_stepper_2pin<P1, P2, D>(
    p1: P1,
    p2: P2,
    delay: D,
    number_of_steps: u32,
) -> Stepper<StepperMotor2<P1, P2>, D>
where
    P1: OutputPin,
    P2: OutputPin<Error = P1::Error>,
    D: DelayNs,
{
    let motor = StepperMotor2 { p1, p2 };
    Stepper::new(number_of_steps, motor, delay)
}

/// Build a 4-pin stepper controller.
pub fn create_stepper_4pin<P1, P2, P3, P4, D>(
    p1: P1,
    p2: P2,
    p3: P3,
    p4: P4,
    delay: D,
    number_of_steps: u32,
) -> Stepper<StepperMotor4<P1, P2, P3, P4>, D>
where
    P1: OutputPin,
    P2: OutputPin<Error = P1::Error>,
    P3: OutputPin<Error = P1::Error>,
    P4: OutputPin<Error = P1::Error>,
    D: DelayNs,
{
    let motor = StepperMotor4 { p1, p2, p3, p4 };
    Stepper::new(number_of_steps, motor, delay)
}

/// Build a 5-pin stepper controller
pub fn create_stepper_5pin<P1, P2, P3, P4, P5, D>(
    p1: P1,
    p2: P2,
    p3: P3,
    p4: P4,
    p5: P5,
    delay: D,
    number_of_steps: u32,
) -> Stepper<StepperMotor5<P1, P2, P3, P4, P5>, D>
where
    P1: OutputPin,
    P2: OutputPin<Error = P1::Error>,
    P3: OutputPin<Error = P1::Error>,
    P4: OutputPin<Error = P1::Error>,
    P5: OutputPin<Error = P1::Error>,
    D: DelayNs,
{
    let motor = StepperMotor5 { p1, p2, p3, p4, p5 };
    Stepper::new(number_of_steps, motor, delay)
}