//! 2-pin stepper coil driver with no error handling

use embedded_hal::digital::OutputPin;
use crate::driver::StepperMotor;

/// Coil driver using two output pins.
pub struct StepperMotor2<P1: OutputPin, P2: OutputPin<Error = P1::Error>> {
    pub(crate) p1: P1,
    pub(crate) p2: P2,
}

impl<P1, P2> StepperMotor for StepperMotor2<P1, P2>
where
    P1: OutputPin,
    P2: OutputPin<Error = P1::Error>,
{
    type Error = ();

    /// Energize pins for the step index `this_step % 4`.
    fn step(&mut self, this_step: u32) -> Result<(), Self::Error> {
        match this_step % 4 {
            0 => {
                let _ = self.p1.set_low();
                let _ = self.p2.set_high();
            }
            1 => {
                let _ = self.p1.set_high();
                let _ = self.p2.set_high();
            }
            2 => {
                let _ = self.p1.set_high();
                let _ = self.p2.set_low();
            }
            3 => {
                let _ = self.p1.set_low();
                let _ = self.p2.set_low();
            }
            _ => unreachable!(),
        }
        Ok(())
    }

    /// Drive all pins low to de-energize the motor.
    fn clear(&mut self) -> Result<(), Self::Error> {
        let _ = self.p1.set_low();
        let _ = self.p2.set_low();
        Ok(())
    }
}
