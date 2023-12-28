
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
    
    let u235_group: Vec<hdf5::Group> = file.as_group()?.groups()?;
    let subgroups = u235_group.first().unwrap().groups()?;

    //dbg!(&u235_group);
    //dbg!(&subgroups);


    Ok(())
}
#[test]
pub fn read_u235_dataset_names() -> Result<(), crate::teh_o_error::TehOError>{

    let file = hdf5::File::open("./src/lib/simulation/monte_carlo/openmc/openmc_nuclides/U235.h5")?;
    // let's try to get a list of folders available
    
    let u235_group: Vec<hdf5::Group> = file.as_group()?.groups()?;
    let subgroups = u235_group.first().unwrap().groups()?;

    for group in subgroups.iter() {
        //dbg!(&group.name());
        //dbg!(group.member_names()?);
    }


    Ok(())
}

#[test]
pub fn read_u235_energy_dataset() -> Result<(), crate::teh_o_error::TehOError>{

    let file = hdf5::File::open("./src/lib/simulation/monte_carlo/openmc/openmc_nuclides/U235.h5")?;
    // let's try to get a list of folders available
    
    let u235_group: Vec<hdf5::Group> = file.as_group()?.groups()?;
    let subgroups = u235_group.first().unwrap().groups()?;

    let energy_subgroup = subgroups.first().unwrap().clone();
    dbg!(&energy_subgroup.name());

    let energy_datasets = energy_subgroup.datasets()?;

    // I can access lists of names by index, but it is not a key/value 

    for dataset_ptr in energy_datasets {

        //dbg!(&dataset_ptr);
        //dbg!(&dataset_ptr.name());
        //dbg!(&dataset_ptr.id());
    }

    

    Ok(())
}

#[test]
pub fn read_u235_energy_dataset_by_name() -> Result<(), crate::teh_o_error::TehOError>{

    let file = hdf5::File::open("./src/lib/simulation/monte_carlo/openmc/openmc_nuclides/U235.h5")?;
    // let's try to get a list of folders available
    let root_group = file.as_group()?;
    assert_eq!(&root_group.name(), "/");

    let u235_groups = root_group.groups()?;
    let u235_group = u235_groups.first().unwrap();
    assert_eq!(&u235_group.name(), "/U235");

    let energy_subgroup: hdf5::Group = u235_group.group("/U235/energy")?;
    assert_eq!(&energy_subgroup.name(), "/U235/energy");

    let room_temp_dataset = energy_subgroup.dataset("/U235/energy/294K")?;
    assert_eq!(&room_temp_dataset.name(), "/U235/energy/294K");

    Ok(())
}
