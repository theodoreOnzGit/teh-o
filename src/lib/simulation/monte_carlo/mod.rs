use crate::teh_o_error::TehOError;

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
    //
    // openmc_simulation_init()
    

    Ok(())

}
