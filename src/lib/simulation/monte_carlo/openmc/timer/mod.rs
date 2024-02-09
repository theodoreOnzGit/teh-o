use std::time::SystemTime;
use uom::ConstZero;
use uom::si::time::nanosecond;
use uom::si::f64::*;

use crate::teh_o_error::TehOError;


/// Timer class translated from OpenMC
#[derive(Debug,PartialEq, PartialOrd, Clone, Copy)]
pub struct Timer{
    running: bool,
    elapsed_time: Time,
    start: SystemTime,
}

impl Default for Timer {
    fn default() -> Self {
        Self { running: false, 
            elapsed_time: Time::ZERO, 
            start: SystemTime::now()
        }
    }
}

impl Timer {
    pub fn start(&mut self){
        self.running = true;
        self.start = SystemTime::now();
    }

    pub fn elapsed(&mut self) -> Result<Time, TehOError> {

        if self.running == true {
            let time_elapsed_ns: f64 = self.start.elapsed()?.as_nanos() as f64;
            self.elapsed_time = Time::new::<nanosecond>(time_elapsed_ns);
            return Ok(self.elapsed_time);
        } else {
            return Ok(self.elapsed_time);
        }
    }

    pub fn stop(&mut self) -> Result<(), TehOError>{
        self.elapsed_time = self.elapsed()?;
        self.running =  false;

        Ok(())
    }

    pub fn reset(&mut self){
        self.running =  false;
        self.elapsed_time = Time::new::<nanosecond>(0.0);
    }

}

