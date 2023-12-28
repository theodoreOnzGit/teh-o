
#[test]
pub fn read_u235_h5() -> Result<(), crate::teh_o_error::TehOError>{

    let file = hdf5::File::open("./src/lib/simulation/monte_carlo/openmc/openmc_nuclides/U235.h5")?;
    let ds = file.dataset("/U235/energy/294K");
    dbg!(&ds);
    Ok(())
}
