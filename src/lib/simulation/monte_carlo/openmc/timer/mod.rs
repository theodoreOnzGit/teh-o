use std::time::SystemTime;
use uom::si::f64::*;

use crate::teh_o_error::TehOError;


/// Timer class translated from OpenMC
pub struct Timer{
    running: bool,
    elapsed: Time,
    start: SystemTime,
}

impl Timer {
    pub fn start(&mut self){
        self.running = true;
        self.start = SystemTime::now();
    }

    pub fn elapsed(&mut self) -> Result<Time, TehOError> {
        let time_elapsed_ns: f64 = self.start.elapsed()?.as_nanos() as f64;

        return Ok(self.elapsed);
    }

    pub fn stop(&mut self){

    }

    pub fn reset(&mut self){

    }

}
