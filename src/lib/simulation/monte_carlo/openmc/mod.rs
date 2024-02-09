
pub mod timer;

/// this helps represent the simulation namespace within 
/// openmc
pub mod open_mc_simulation_environment;
pub mod openmc_nuclides;
pub mod openmc_hdf5_interface;
pub mod random_dist;
/// linear congruential generator
pub mod random_lcg;

/// math functions, legendre polynomials and the like
pub mod math_functions;

/// struct representing position of particle 
pub mod position;

/// structs represening surfaces 
pub mod surface;

/// structs representing particle data
pub mod particle_data;
pub mod particle;

use crate::teh_o_error::TehOError;

use open_mc_simulation_environment::{OpenMCSimulationEnvironmentVariables, openmc_settings::RunMode};
/// this function encompasses all the main 
/// logic where iterations are performed over batches, iterations, and 
/// histories in a fixed source or k-eigenvalue calculation
///
/// This was based on translating openmc code written in c or cpp into 
/// Rust 
///
/// todo: probably change String Error to proper error
pub fn openmc_run() -> Result<(),TehOError>{

    // openmc::simulation::time_total::start();
    let mut simulation = open_mc_simulation_environment::
        OpenMCSimulationEnvironmentVariables::default();
    simulation.time_total.start();
    
    // openmc_simulation_init()
    
    // openmc::simulation::time_total::stop();
    simulation.time_total.stop()?;
    Ok(())

}

fn openmc_simulation_init(
    sim_environment: OpenMCSimulationEnvironmentVariables) -> 
Result<(), TehOError>{
    
    // Skip if simulation has already been initialised
    if sim_environment.initialised {
        return Ok(());
    }

    // initialise nuclear data if running in continuous energy 
    // mode (run_CE) in original openmc mode
    if let Some(true) =
        sim_environment.settings.run_continuous_energy {

        }

    Ok(())
}

fn initialise_data(
    sim_environment: OpenMCSimulationEnvironmentVariables)
    -> Result<(),TehOError>{


        Ok(())
    }
