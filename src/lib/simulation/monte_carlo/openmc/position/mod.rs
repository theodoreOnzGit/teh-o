use uom::si::f64::*;

use crate::teh_o_error::TehOError;
pub mod math_ops;
pub mod vector_ops;


#[derive(Debug,PartialEq, PartialOrd, Copy, Clone)]
pub struct Position {
    x: Length,
    y: Length,
    z: Length,
}

// For OpenMC, I found that it does poorly with TRISO particles due 
// to its surface tracking algorithm. In surface tracking,
// for each particle, OpenMC has to perform a calculation for 
// each and every surface. That is to check the distance from them 
// and/or check where the particle is with respect to the surface.
//
// That is, it will check if the particle is (x,y,z) and then substitute 
// that into the surface equation. This is done for each surface.
//
// For a single pebble, we have roughly 20,000 TRISO particles. Each 
// TRISO particle has about 5 layers or surfaces. This means each 
// pebble has about 100,000 surfaces in consideration. For a pebble bed 
// with doubly heterogeneous geometry, we have several thousand pebbles.
//
// Suppose there are 10,000 pebbles in a pebble bed. We would then have 
// to track roughly 1 billion surfaces. This would require lots of memory.
// To estimate memory for this is difficult without prior experience. 
// But we can use CFD to guess how much memory is needed. 
// As a rule of thumb, 1 million cells in a CFD mesh requires about 
// 3 GB of RAM. In other works, each cell takes about 3 kB of memory.
// Assume for now that each surface requires about 100 bytes of memory 
// at the low bound since it will need memory allocated for functions 
// and variables to calculate the surface equations. We may need roughly 
// 100 GB of RAM just for a pebble bed for each pebble.
//
// We can use lattices and universes to reduce the number of explicit 
// surfaces we need to model. This is shown in the TRISO particles 
// python notebook.
//
// https://github.com/openmc-dev/openmc-notebooks/blob/main/triso.ipynb
//
// Nevertheless, my experience even with this method is that full pebble 
// beds cannot be modelled. We need something new.
//
// How can we model particles such that we don't always need to check 
// their positions?
//
//
// One idea is to have each cell indicate if a particle is inside them.
// The particle has to exit through one of the cell surfaces to an 
// adjacent cell or boundary condition. We hope that each cell has 
// a small number of surfaces such that we only need to track the 
// particle with respect to a limited number of surfaces. 
//
// Based on particle trajectory, the particle would than intersect 
// with a particular surface and be passed on to the next cell through 
// that said surface. In this manner, we track cells rather than track 
// the particles. Hopefully, the number of times we need to track surfaces 
// would then decrease dramatically.
//
// One argument against this is perhaps in the use of lattices. When we 
// consider lattices, we also try to reduce the number of times we track 
// surfaces. We essentially track particles with respect to a cubic 
// lattice which is easy to track. And then, within each lattice, we track 
// if the particle is in one of the cells. Therefore, we skip surface 
// tracking for many irrelevant particles. This optimisation, however,
// was insufficient in helping us track particles in a quick enough manner.
//
// Another argument against this method is perhaps that when we consider 
// the graphite matrix, the graphite matrix is shaped in a very 
// irregular manner, such that it has many adjacent surfaces. Each TRISO 
// particle represents one adjacent surface. So we have about 10,000 surfaces 
// to track. This brings us back to square one.
//
// The last argument is that Serpent's delta tracking is already tried 
// and proven. Why not do it?
//
// For the first argument, we could perhaps consider in lattice tracking,
// we have several lattices to track. Firs the outer lattice of the pebble 
// bed, then the inner lattice of the TRISO particle. If we have 1000 
// cells to track in each lattice, we still have at least 2000 lattice 
// tracking operations. However, when tracking adjacent cells, we only 
// need to track a more limited number of adjacent cells (hopefully on 
// the order of 10 or 20).
//
// Nevertheless, when the particle exists in the graphite matrix, we then 
// need to perform surface tracking with about 10,000 surfaces adjacent 
// to the graphite. This is not suitable. We could, of course, adopt 
// the lattice method in order to reduce the number of adjacent surfaces 
// we need to track just as was done in OpenMC. We then track a subset 
// of cells or surfaces within the lattice.
//
// For the last argument, I think serpent's delta tracking eliminates 
// the possibility of using the track length estimator. This is a suitable 
// trade-off to make pebble beds do-able. However, in terms of generating 
// MGXS, this could be problematic as we need more particles to estimate 
// MGXS. For running these simulations, MGXS generation is a key mission 
// of the Monte Carlo code.
//
// Probably want to conduct a literature review on what methods have 
// been explored for this issue with TRISO pebble beds. I think 
// this is super interesting to explore.
//
//
// 

impl Position {
    
    pub fn get(&self, index: u8) -> Result<Length, TehOError>{

        let quantity = match index {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => return Err(TehOError::OpenMcErrOutOfBounds),
        };

        return Ok(quantity);
    }

}


