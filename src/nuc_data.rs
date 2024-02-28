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
fn main() {
    println!("Hello, world!");
}
