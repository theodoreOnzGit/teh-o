
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

#[test]
pub fn read_u235_folder() -> Result<(), crate::teh_o_error::TehOError>{

    let file = hdf5::File::open("./src/lib/simulation/monte_carlo/openmc/openmc_nuclides/U235.h5")?;
    // let's try to get a list of folders available
    
    let names_of_groups = file.member_names()?;
    assert_eq!(names_of_groups[0],"U235");

    Ok(())
}
#[test]
pub fn read_u235_subgroups() -> Result<(), crate::teh_o_error::TehOError>{

    let file = hdf5::File::open("./src/lib/simulation/monte_carlo/openmc/openmc_nuclides/U235.h5")?;
    // let's try to get a list of folders available
    
    let main_group: hdf5::Group = file.as_group()?;
    let u235_group: Vec<hdf5::Group> = main_group.groups()?;
    let subgroups = u235_group[0].groups()?;

    dbg!(&u235_group);
    dbg!(&subgroups);


    Ok(())
}
