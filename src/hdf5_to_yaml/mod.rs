use std::fs::File;
use std::io::Write;

use serde::Deserialize;
use serde::Serialize;
use serde_yaml::Sequence;
use uom::si::f64::*;
use uom::si::thermodynamic_temperature::kelvin;

#[derive(Serialize,Debug,Deserialize)]
pub struct FissionXsYaml {
    energy_levels_ev: serde_yaml::Sequence,
    xs_barns: serde_yaml::Sequence
}

pub fn get_nuclide_xs_at_temperature(nuclide: &str,
    reaction_mt_number: &str,
    temperature_kelvin: &str,)-> Result<(), teh_o::teh_o_error::TehOError>{

    // for serialisation to yaml, it is best to use the sequence 
    // as it is a vector of yaml values
    // 

    use serde_yaml::Value;

    let file = hdf5::File::open(
        "./src/lib/simulation/monte_carlo/openmc/openmc_nuclides/".to_owned()+nuclide+".h5")?;
    let ds_energy_levels = file.dataset(&("/".to_owned()+nuclide+"/energy/"+
            temperature_kelvin+"K"))?;
    let nuclide_energy_array = ds_energy_levels.read_1d::<f64>()?;

    //// this shows energy in eV
    //dbg!(&nuclide_energy_array);


    let nuclide_group: Vec<hdf5::Group> = file.as_group()?.groups()?;
    let subgroups = nuclide_group.first().unwrap().groups()?;

    for group in subgroups.iter() {
        //dbg!(&group.name());
        //dbg!(group.member_names()?);
    }

    // hdf5 group for fission cross section
    let group_cross_sections_n_fission = file.group(
        &("/".to_owned()+nuclide+"/reactions/reaction_"+reaction_mt_number+"/"
            +temperature_kelvin+"K"))?;

    //dbg!(&group_cross_sections_n_fission);
    //https://t2.lanl.gov/nis/endf/mts.html
    //includes first, second etc. fissions

    let n_f_member_names = group_cross_sections_n_fission.member_names()?;

    //dbg!(&n_f_member_names);

    let fission_dataset = group_cross_sections_n_fission.dataset("xs")?;
    //dbg!(&fission_dataset);
    let nuclide_fission_array_desired_temp_k = fission_dataset.read_1d::<f64>()?;

    //dbg!(&nuclide_fission_array_desired_temp_K);

    // now convert this to a toml file
    // to do so, I need to convert the energy and fission arrays into 
    // a toml readable format


    let intermediate_nuclide_energy_ev_float_vec: Vec<f64> = nuclide_energy_array.iter().map(
        |energy_ev|{
            *energy_ev
        }
    ).collect();

    // Sequence is a type alias for Vec<Value>
    let yaml_energy_nuclide_energy_array: Sequence = 
        intermediate_nuclide_energy_ev_float_vec.into_iter().map(
            |value_f64_ref|{
                Value::Number(value_f64_ref.into())
            }).collect();

    let intermediate_nuclide_reaction_xs_barns_float_vec: Vec<f64> = nuclide_fission_array_desired_temp_k.iter().map(
        |n_reaction_xs_barns|{
            *n_reaction_xs_barns
        }
    ).collect();

    // Sequence is a type alias for Vec<Value>
    let yaml_reaction_xs_nuclide_desired_temp_k_array: Vec<Value> = 
        intermediate_nuclide_reaction_xs_barns_float_vec.into_iter().map(
            |value_f64_ref|{
                Value::Number(value_f64_ref.into())
            }).collect();


    let reaction_xs_yaml_desired_temp_k: FissionXsYaml = FissionXsYaml { 
        energy_levels_ev: yaml_energy_nuclide_energy_array, 
        xs_barns: yaml_reaction_xs_nuclide_desired_temp_k_array
    };

    let yaml_serialised = serde_yaml::to_string(&reaction_xs_yaml_desired_temp_k).unwrap();

    //dbg!(&yaml_serialised);

    // lets convert this to u8

    let yaml_u8_string: Vec<u8> = yaml_serialised.into(); 

    let mut nuclide_desired_temp_k_xs_test = File::create(
        nuclide.to_owned()
        +"_mt"
        + reaction_mt_number+"_"
        +temperature_kelvin
        +"K.yml")?;
    nuclide_desired_temp_k_xs_test.write_all(&yaml_u8_string)?;

    Ok(())
}

pub fn get_nuclide_xs_all_temp(nuclide: &str,
    reaction_mt_number: &str)-> 
Result<(), teh_o::teh_o_error::TehOError>{
    let mut temperatures: Vec<ThermodynamicTemperature> = vec![];

    temperatures.push(ThermodynamicTemperature::new::<kelvin>(250.0));
    temperatures.push(ThermodynamicTemperature::new::<kelvin>(294.0));
    temperatures.push(ThermodynamicTemperature::new::<kelvin>(600.0));
    temperatures.push(ThermodynamicTemperature::new::<kelvin>(900.0));
    temperatures.push(ThermodynamicTemperature::new::<kelvin>(1200.0));
    temperatures.push(ThermodynamicTemperature::new::<kelvin>(2500.0));

    for temperature_ref in temperatures {

        let temperature_float: f64 = temperature_ref.get::<kelvin>();
        let temperature_int: u64 = temperature_float as u64;
        let temperature_str: &str = &temperature_int.to_string();

        get_nuclide_xs_at_temperature(nuclide, reaction_mt_number, 
            temperature_str)?;

    }


    Ok(())
}
