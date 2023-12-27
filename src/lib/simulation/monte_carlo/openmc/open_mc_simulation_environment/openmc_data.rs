use uom::si::f64::*;
use uom::si::energy::electronvolt;

#[derive(Debug,PartialEq,Clone)]
pub struct OpenMCData {
    energy_min: [Energy;2],
    energy_max: [Energy;2],
}

impl Default for OpenMCData {
    fn default() -> Self {
        Self { 
            energy_min: [Energy::new::<electronvolt>(0.0);2], 
            energy_max: [Energy::new::<electronvolt>(f64::MAX);2] ,

        }
    }
}
