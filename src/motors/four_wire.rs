//! 4-pin stepper coil driver with no error handling

use embedded_hal::digital::OutputPin;
use crate::driver::StepperMotor;

//// Coil driver using four output pins.
pub struct StepperMotor4<P1: OutputPin, P2: OutputPin, P3: OutputPin, P4: OutputPin>
where
    P1: OutputPin,
    P2: OutputPin<Error = P1::Error>,
    P3: OutputPin<Error = P1::Error>,
    P4: OutputPin<Error = P1::Error>,
{
    pub p1: P1,
    pub p2: P2,
    pub p3: P3,
    pub p4: P4,
}

impl<P1, P2, P3, P4> StepperMotor for StepperMotor4<P1, P2, P3, P4>
where
    P1: OutputPin,
    P2: OutputPin<Error = P1::Error>,
    P3: OutputPin<Error = P1::Error>,
    P4: OutputPin<Error = P1::Error>,
{
    type Error = ();

    /// Energize pins for the step index `this_step % 4`.
    fn step(&mut self, this_step: u32) -> Result<(), Self::Error> {
        match this_step % 4 {
            0 => {
                let _ = self.p1.set_high();
                let _ = self.p2.set_low();
                let _ = self.p3.set_high();
                let _ = self.p4.set_low();
            }
            1 => {
                let _ = self.p1.set_low();
                let _ = self.p2.set_high();
                let _ = self.p3.set_high();
                let _ = self.p4.set_low();
            }
            2 => {
                let _ = self.p1.set_low();
                let _ = self.p2.set_high();
                let _ = self.p3.set_low();
                let _ = self.p4.set_high();
            }
            3 => {
                let _ = self.p1.set_high();
                let _ = self.p2.set_low();
                let _ = self.p3.set_low();
                let _ = self.p4.set_high();
            }
            _ => unreachable!(),
        }
        Ok(())
    }

    /// Drive all pins low to de-energize the motor.
    fn clear(&mut self) -> Result<(), Self::Error> {
        let _ = self.p1.set_low();
        let _ = self.p2.set_low();
        let _ = self.p3.set_low();
        let _ = self.p4.set_low();
        Ok(())
    }
}
