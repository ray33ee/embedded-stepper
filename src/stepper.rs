//! High-level **blocking** stepper controller ported directly from the Arduino stepper library.
//!
//! This struct delegates coil energizing to a [`StepperMotor`]
//! implementation and uses an [`DelayNs`] for timing.

use embedded_hal::delay::DelayNs;
use crate::driver::StepperMotor;

/// High-level stepper controller parameterized by a motor coil driver `M` and a delay `D`.
pub struct Stepper<M: StepperMotor, D: DelayNs> {
    pub(crate) motor: M,
    pub(crate) delay: D,
    step_number: u32,
    direction: u32,
    step_delay: u32,
    number_of_steps: u32,
}

impl<M: StepperMotor, D: DelayNs> Stepper<M, D> {
    /// Create a new controller.
    ///
    /// - `number_of_steps`: steps per mechanical revolution (as in Arduino stepper constructor)
    /// - `motor`: coil driver implementing [`StepperMotor`]
    /// - `delay`: timing provider implementing [`DelayNs`]
    pub fn new(number_of_steps: u32, motor: M, delay: D) -> Self {
        Self {
            motor,
            delay,
            step_number: 0,
            direction: 0,
            step_delay: 0,
            number_of_steps,
        }
    }

    /// Set speed in **RPM**.
    ///
    /// Equivalent to the Arduino stepper function with the same name except the division by 0 check.
    ///
    /// If `speed` or `number_of_steps` are 0, the speed is set to 0
    pub fn set_speed(&mut self, speed: u32) {
        if speed != 0 && self.number_of_steps != 0 {
            self.step_delay = 60 * 1_000_000 / self.number_of_steps / speed;
        } else {
            self.step_delay = 0;
        }
    }

    /// De-energize all coils by delegating to [clear](`StepperMotor::clear`).
    pub fn deenergise(&mut self) -> Result<(), M::Error> {
        self.motor.clear()
    }

    /// Perform a **blocking** move of `steps_to_move` steps.
    ///
    /// Positive values step forward; negative values step backward.
    ///
    /// Ported from Arduino stepper `step` function
    pub fn step(&mut self, steps_to_move: i32) -> Result<(), M::Error> {
        let mut steps_left = if steps_to_move < 0 {
            self.direction = 0;
            -steps_to_move as u32
        } else {
            self.direction = 1;
            steps_to_move as u32
        };

        while steps_left > 0 {
            self.delay.delay_us(self.step_delay);

            if self.direction == 1 {
                self.step_number += 1;
                if self.step_number == self.number_of_steps {
                    self.step_number = 0;
                }
            } else {
                if self.step_number == 0 {
                    self.step_number = self.number_of_steps;
                }
                self.step_number -= 1;
            }

            steps_left -= 1;

            self.motor.step(self.step_number)?;
        }

        Ok(())
    }
}
