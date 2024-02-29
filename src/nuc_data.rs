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

    read_u235_data_to_toml().unwrap();

}


// in this test, i want to read U235 (n,f) cross sections and plot them 
// out into a toml file
pub fn read_u235_data_to_toml() -> Result<(), teh_o::teh_o_error::TehOError>{

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

    let n_f_member_names = group_cross_sections_n_fission.member_names()?;

    //dbg!(&n_f_member_names);

    let fission_dataset = group_cross_sections_n_fission.dataset("xs")?;
    //dbg!(&fission_dataset);
    let u235_fission_array_294K = fission_dataset.read_1d::<f64>()?;

    //dbg!(&u235_fission_array_294K);

    Ok(())
}

