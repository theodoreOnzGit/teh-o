
#[test]
pub fn open_u235_h5() -> Result<(), crate::teh_o_error::TehOError>{

    let file = hdf5::File::open("./src/lib/simulation/monte_carlo/openmc/openmc_nuclides/U235.h5")?;
    let ds = file.dataset("/U235/energy/294K")?;
    dbg!(&ds);
    Ok(())
}

#[test]
pub fn read_u235_data() -> Result<(), crate::teh_o_error::TehOError>{

    let file = hdf5::File::open("./src/lib/simulation/monte_carlo/openmc/openmc_nuclides/U235.h5")?;
    let ds = file.dataset("/U235/energy/294K")?;
    let u235_energy_float = ds.read_1d::<f64>()?;

    let energy_level = u235_energy_float.get(1).unwrap();
    
    assert_eq!(*energy_level,1.03125e-5);
    Ok(())
}
