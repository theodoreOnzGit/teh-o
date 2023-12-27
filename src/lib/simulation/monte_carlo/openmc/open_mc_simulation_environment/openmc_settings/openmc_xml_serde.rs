use serde::{Deserialize, Serialize};

use std::{fs, path::PathBuf};

use crate::teh_o_error::TehOError;

pub fn read_file(filepath: PathBuf)-> Result<String, TehOError>{
    let file_contents = fs::read_to_string(filepath)?;

    Ok(file_contents)
}


/// type for openmc settings file
#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Settings {
    run_mode: String,
    particles: i32,
    batches: i32,
    inactive: i32,
    source: OpenMCSource
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct OpenMCSource {
    particle: String,
    strength: String,
    r#type: OpenMCSourceType,
    space: OpenMCSourceSpace,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[allow(non_camel_case_types)]
enum OpenMCSourceType {
    independent,
}


#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[allow(non_camel_case_types)]
struct OpenMCSourceSpace {
    r#type: OpenMCSpaceType,
    parameters: String,
}


#[derive(Debug, Serialize, Deserialize, PartialEq)]
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
    assert_eq!(settings.source.space.parameters, "-10.71 -10.71 -1 10.71 10.71 1");

}

