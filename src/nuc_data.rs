use std::fs::File;
use std::io::Write;

use serde::Deserialize;
use serde::Serialize;
use serde_yaml::Sequence;

// This binary is responsible for converting the entire hdf5 library 
// into a yaml format because hdf5 libraries in Rust are quite 
// problematic 
//
// https://towardsdatascience.com/which-data-format-to-use-for-your-big-data-project-837a48d3661d
//
// For serialisation and deserialisation, it seems hdf5 and pickle 
// are the fastest. This is important because read and write speeds may 
// impact continuous cross section reading times and therefore 
// calculation times
//
// However, for Rust crates, pickle is not yet optimised, almost as slow 
// as json
// https://github.com/birkenfeld/serde-pickle/issues/14
//
// of all the rust crates, toml data types are most optimised with 
// 142 million downloads (as of feb 2024), which decreases risks of bugs
// https://crates.io/crates/toml 
//
// yaml on the other hand is also useful and mature with approximately 
// 52 million downloads
//
// https://crates.io/crates/serde_yaml
//
// json is the most used. with 229 million downloads as of feb 2024
//
// I don't like json file formats because of its untidiness. I suppose 
// toml works a little better given its popularity and nativeness in Rust. 
//
// Probably not going to be as fast as hdf5, but decent
//
// for this, I want to deserialise all my hdf5 files from openmc 
// and change them into toml files suitable for Rust, then I want to 
// make a new crate which is independent of hdf5 files and only uses toml 
// files.
//
//
fn main() {
    println!("Hello, world!");

    let toggle_toml = false;
    let toggle_yaml = true;
    if toggle_toml {
        read_u235_data_to_toml().unwrap();
    }

    if toggle_yaml {
        read_u235_data_to_yaml().unwrap();
    }



}

// in this test, I want to read U235 (n,f) cross sections and plot them to 
// a yaml file. This is because cargo watch -x run does not work well with 
// --ignore *.toml
pub fn read_u235_data_to_yaml()-> Result<(), teh_o::teh_o_error::TehOError>{

    // for serialisation to yaml, it is best to use the sequence 
    // as it is a vector of yaml values
    // 
    use serde_yaml::Value;

    let file = hdf5::File::open("./src/lib/simulation/monte_carlo/openmc/openmc_nuclides/U235.h5")?;
    let ds_energy_levels = file.dataset("/U235/energy/294K")?;
    let u235_energy_array = ds_energy_levels.read_1d::<f64>()?;

    //// this shows energy in eV
    //dbg!(&u235_energy_array);


    let u235_group: Vec<hdf5::Group> = file.as_group()?.groups()?;
    let subgroups = u235_group.first().unwrap().groups()?;

    for group in subgroups.iter() {
        //dbg!(&group.name());
        //dbg!(group.member_names()?);
    }

    // hdf5 group for fission cross section
    let group_cross_sections_n_fission = file.group("/U235/reactions/reaction_018/294K")?;

    //dbg!(&group_cross_sections_n_fission);
    //https://t2.lanl.gov/nis/endf/mts.html
    //includes first, second etc. fissions

    let n_f_member_names = group_cross_sections_n_fission.member_names()?;

    //dbg!(&n_f_member_names);

    let fission_dataset = group_cross_sections_n_fission.dataset("xs")?;
    //dbg!(&fission_dataset);
    let u235_fission_array_294K = fission_dataset.read_1d::<f64>()?;

    //dbg!(&u235_fission_array_294K);
    
    // now convert this to a toml file
    // to do so, I need to convert the energy and fission arrays into 
    // a toml readable format


    let intermediate_u235_energy_ev_float_vec: Vec<f64> = u235_energy_array.iter().map(
            |energy_ev|{
                *energy_ev
            }
            ).collect();

    // Sequence is a type alias for Vec<Value>
    let yaml_energy_u235_energy_array: Sequence = 
        intermediate_u235_energy_ev_float_vec.into_iter().map(
        |value_f64_ref|{
            Value::Number(value_f64_ref.into())
        }).collect();

    let intermediate_u235_fission_xs_barns_float_vec: Vec<f64> = u235_fission_array_294K.iter().map(
            |n_fission_xs_barns|{
                *n_fission_xs_barns
            }
            ).collect();

    // Sequence is a type alias for Vec<Value>
    let yaml_fission_xs_u235_294_k_array: Vec<Value> = 
        intermediate_u235_fission_xs_barns_float_vec.into_iter().map(
        |value_f64_ref|{
            Value::Number(value_f64_ref.into())
        }).collect();


    let fission_xs_yaml_294_k: FissionXsYaml = FissionXsYaml { 
        energy_levels_ev: yaml_energy_u235_energy_array, 
        fission_xs_barns: yaml_fission_xs_u235_294_k_array
    };

    let yaml_serialised = serde_yaml::to_string(&fission_xs_yaml_294_k).unwrap();

    //dbg!(&yaml_serialised);

    // lets convert this to u8

    let yaml_u8_string: Vec<u8> = yaml_serialised.into(); 

    let mut u235_294k_xs_test = File::create("u235_mt18_fission_294K.yml")?;
    u235_294k_xs_test.write_all(&yaml_u8_string)?;

    Ok(())
}

#[derive(Serialize,Debug,Deserialize)]
pub struct FissionXsYaml {
    energy_levels_ev: serde_yaml::Sequence,
    fission_xs_barns: serde_yaml::Sequence
}

// in this test, i want to read U235 (n,f) cross sections and plot them 
// out into a toml file
pub fn read_u235_data_to_toml() -> Result<(), teh_o::teh_o_error::TehOError>{

    use toml::Value;
    let file = hdf5::File::open("./src/lib/simulation/monte_carlo/openmc/openmc_nuclides/U235.h5")?;
    let ds_energy_levels = file.dataset("/U235/energy/294K")?;
    let u235_energy_array = ds_energy_levels.read_1d::<f64>()?;

    //// this shows energy in eV
    //dbg!(&u235_energy_array);


    let u235_group: Vec<hdf5::Group> = file.as_group()?.groups()?;
    let subgroups = u235_group.first().unwrap().groups()?;

    for group in subgroups.iter() {
        //dbg!(&group.name());
        //dbg!(group.member_names()?);
    }

    // hdf5 group for fission cross section
    let group_cross_sections_n_fission = file.group("/U235/reactions/reaction_018/294K")?;

    //dbg!(&group_cross_sections_n_fission);
    //https://t2.lanl.gov/nis/endf/mts.html
    //includes first, second etc. fissions

    let n_f_member_names = group_cross_sections_n_fission.member_names()?;

    //dbg!(&n_f_member_names);

    let fission_dataset = group_cross_sections_n_fission.dataset("xs")?;
    //dbg!(&fission_dataset);
    let u235_fission_array_294K = fission_dataset.read_1d::<f64>()?;

    //dbg!(&u235_fission_array_294K);
    
    // now convert this to a toml file
    // to do so, I need to convert the energy and fission arrays into 
    // a toml readable format


    let intermediate_u235_energy_ev_float_vec: Vec<f64> = u235_energy_array.iter().map(
            |energy_ev|{
                *energy_ev
            }
            ).collect();

    let toml_energy_u235_energy_array: Vec<Value> = 
        intermediate_u235_energy_ev_float_vec.into_iter().map(
        |value_f64_ref|{
            Value::Float(value_f64_ref)
        }).collect();


    let intermediate_u235_fission_xs_barns_float_vec: Vec<f64> = u235_fission_array_294K.iter().map(
            |n_fission_xs_barns|{
                *n_fission_xs_barns
            }
            ).collect();

    let toml_fission_xs_u235_294K_array: Vec<Value> = 
        intermediate_u235_fission_xs_barns_float_vec.into_iter().map(
        |value_f64_ref|{
            Value::Float(value_f64_ref)
        }).collect();

    // export to toml file 
    let fission_xs_toml_294K: FissionXsToml = FissionXsToml { 
        energy_levels_ev: toml_energy_u235_energy_array, 
        fission_xs_barns: toml_fission_xs_u235_294K_array
    };
    
    let toml_serialised = toml::to_string(&fission_xs_toml_294K).unwrap();

    dbg!(&toml_serialised);

    // lets convert this to u8

    let toml_u8_string: Vec<u8> = toml_serialised.into(); 

    // i find that toml is quite unwieldy to work with cargo watch

    
    let mut u235_294k_xs_test = File::create("u235_mt18_fission_294K.toml")?;
    u235_294k_xs_test.write_all(&toml_u8_string)?;

    Ok(())
}


#[derive(Serialize,Debug,Deserialize)]
pub struct FissionXsToml {
    energy_levels_ev: toml::value::Array,
    fission_xs_barns: toml::value::Array
}

