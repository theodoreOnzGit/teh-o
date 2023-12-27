use uom::si::f64::*;

#[derive(Debug,Default,PartialEq, Clone)]
pub struct OpenMCNuclide {
    // Types, aliases
    // emission mode (TBD)
    name: Option<String>,

}

#[derive(Debug,Default,PartialEq, Clone)]
struct OpenMCEnergyGrid {
    grid_index: Option<Vec<i32>>,
    energy: Option<Vec<Energy>>,
}

// probably need to interface with hdf5 types
// oof
