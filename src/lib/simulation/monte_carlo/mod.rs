use crate::teh_o_error::TehOError;

use self::openmc::open_mc_simulation_environment::OpenMCSimulationEnvironmentVariables;

pub mod openmc;

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
    let mut simulation = openmc::open_mc_simulation_environment::
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

    Ok(())
}
