use serde::{Deserialize, Serialize};

use std::{fs, path::PathBuf};

use crate::teh_o_error::TehOError;

pub fn read_file(filepath: PathBuf)-> Result<String, TehOError>{
    let file_contents = fs::read_to_string(filepath)?;

    Ok(file_contents)
}


/// type for openmc settings file
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
struct Settings {
    run_mode: String,
    particles: i32,
    batches: i32,
    inactive: i32,
    source: OpenMCSource
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
struct OpenMCSource {
    particle: String,
    strength: String,
    r#type: OpenMCSourceType,
    space: OpenMCSourceSpace,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[allow(non_camel_case_types)]
enum OpenMCSourceType {
    independent,
}


#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[allow(non_camel_case_types)]
struct OpenMCSourceSpace {
    r#type: OpenMCSpaceType,
    parameters: String,
}


#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[allow(non_camel_case_types)]
enum OpenMCSpaceType {
    fission,
}

#[test]
pub fn simplest_serde_test_openmc_xml(){
    use serde_xml_rs::{from_str, to_string};
    let test_settings_xml_filepath: PathBuf = 
        "./src/lib/simulation/monte_carlo/openmc/open_mc_simulation_environment/assembly_settings_example.xml".into();


    let file_contents: String = read_file(test_settings_xml_filepath).unwrap();

    let settings: Settings = from_str(&file_contents).unwrap();

    assert_eq!(settings.particles, 1000);
    assert_eq!(settings.batches, 150);
    assert_eq!(settings.inactive, 50);
    assert_eq!(settings.run_mode, "eigenvalue".to_string());

}

#[test]
pub fn tag_serde_test_openmc_xml(){
    use serde_xml_rs::{from_str, to_string};
    let test_settings_xml_filepath: PathBuf = 
        "./src/lib/simulation/monte_carlo/openmc/open_mc_simulation_environment/assembly_settings_example.xml".into();


    let file_contents: String = read_file(test_settings_xml_filepath).unwrap();

    let settings: Settings = from_str(&file_contents).unwrap();

    assert_eq!(settings.run_mode, "eigenvalue".to_string());
    assert_eq!(settings.source.particle, "neutron".to_string());
    assert_eq!(settings.source.strength, "1.0".to_string());
    assert_eq!(settings.source.r#type, OpenMCSourceType::independent);
    assert_eq!(settings.source.space.r#type, OpenMCSpaceType::fission);
    // this is hard, how do I parse multiple floating point numbers from 
    // this string??
    assert_eq!(settings.source.space.parameters, "-10.71 -10.71 -1 10.71 10.71 1");

}

#[test]
pub fn string_to_float_vec_serde_test_openmc_xml(){
    use serde_xml_rs::{from_str, to_string};
    let test_settings_xml_filepath: PathBuf = 
        "./src/lib/simulation/monte_carlo/openmc/open_mc_simulation_environment/assembly_settings_example.xml".into();


    let file_contents: String = read_file(test_settings_xml_filepath).unwrap();

    let settings: Settings = from_str(&file_contents).unwrap();

    // how do I parse multiple floating point numbers from 
    // this string??
    assert_eq!(settings.source.space.parameters, "-10.71 -10.71 -1 10.71 10.71 1");
    // 
    let coordinates = settings.source.space.parameters.clone();
    let nums = coordinates.trim().split(' ').flat_map(str::parse::<f64>).collect::<Vec<_>>();


    assert_eq!(nums,vec![-10.71,-10.71,-1 as f64,10.71,10.71,1 as f64]);


}

