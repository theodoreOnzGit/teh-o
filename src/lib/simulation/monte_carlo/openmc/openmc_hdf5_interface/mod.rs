use crate::teh_o_error::TehOError;


#[test]
pub fn read_u235_h5() -> Result<(),TehOError>{

    let file = hdf5::File::open("./src/lib/simulation/monte_carlo/openmc/openmc_nuclides/U235.h5")?;
    let ds = file;
    dbg!(&ds);
    Ok(())
}
