use std::collections::HashMap;

use uom::ConstZero;
use uom::si::f64::*;
use uom::si::energy::electronvolt;
use uom::si::thermodynamic_temperature::kelvin;

// equivalent of data namespace in openmc
// for type conversions to Rust
// https://maulingmonkey.com/guide/cpp-vs-rust/
#[derive(Debug,PartialEq,Clone)]
pub struct OpenMCData {
    energy_min: [Energy;2],
    energy_max: [Energy;2],
    temperature_min: ThermodynamicTemperature,
    temperature_max: ThermodynamicTemperature,
    nuclide_map: Option<HashMap<String, i32>>,
}

impl Default for OpenMCData {
    fn default() -> Self {
        Self { 
            energy_min: [Energy::new::<electronvolt>(0.0);2], 
            energy_max: [Energy::new::<electronvolt>(f64::MAX);2],
            // set this as zero because you can compare later
            temperature_min: ThermodynamicTemperature::new::<kelvin>(
                f64::MAX),
            temperature_max: ThermodynamicTemperature::ZERO,
            nuclide_map: None,
        }
    }
}
