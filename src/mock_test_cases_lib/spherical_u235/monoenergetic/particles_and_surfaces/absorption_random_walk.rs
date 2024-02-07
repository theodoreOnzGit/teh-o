use uom::ConstZero;
use uom::si::f64::*;
use uom::si::ratio::ratio;
/// This test simulates one particle's random walk 
/// in an infinite medium with isotropic scattering and an 
/// absorption cross section
#[test]
pub fn test1_random_walk_infinite_medium_absorption(){

    use uom::si::area::barn;
    use uom::si::mass_density::gram_per_cubic_centimeter;
    use uom::si::molar_mass::gram_per_mole;
    use teh_o::simulation::monte_carlo::openmc::random_dist::uniform_distribution;
    // first thing is that we want to simulate one single particle 
    // in an infinite medium with a fixed arbitrary 
    // macroscopic cross section fixed absorption cross section 
    //
    // we simulate the particle essentially until it is absorbed.
    
    // first things first, we need to simulate a particle with x,y,z 
    // coordinates as well as a unit vector indicating its direction
    // here it is: 

    let mut particle_1 = MockTestMonoenergeticParticle::default();

    // now of course, we don't care about the energy of the particle 
    // just yet, but we do care about its cross sections

    // let's consider a u235 textbook example for one group fast neutrons
    //
    // Lamarsh, John R., and Anthony John Baratta. 
    // Introduction to nuclear engineering. Vol. 3. 
    // Upper Saddle River, NJ: Prentice hall, 2001.
    let u235_abs_xs: Area = Area::new::<barn>(1.65);
    let _u235_fiss_xs: Area = Area::new::<barn>(1.4);
    let _u235_n_gamma_xs: Area = Area::new::<barn>(0.25);
    let _u235_transport_xs: Area = Area::new::<barn>(6.8);

    // for scattering, I can take 14 MeV neutrons as a reference 
    // https://wwwndc.jaea.go.jp/cgi-bin/Tab80WWW.cgi?lib=J40&iso=U235
    // ignoring inelastic scattering here
    
    let u235_scatter_xs: Area = Area::new::<barn>(2.839);
    let u235_total_xs: Area = u235_abs_xs + u235_scatter_xs;

    // let's consider the density of u235 metal
    // https://en.wikipedia.org/wiki/Uranium
    // 19.1 g/cm3 for natural uranium 
    // u235 will be slightly lighter but nevermind
    let uranium_density: MassDensity = MassDensity::
        new::<gram_per_cubic_centimeter>(19.1);

    // uranium atom density 
    let uranium_atom_density: VolumetricNumberDensity;
    let uranium_molar_mass: MolarMass = 
        MolarMass::new::<gram_per_mole>(235.0);
    let one_particle: AmountOfSubstance = AmountOfSubstance::
        new::<uom::si::amount_of_substance::particle>(1.0);
    uranium_atom_density = 
        (uranium_density / uranium_molar_mass / one_particle).into();

    let uranium_atom_density_per_cm3 = 
        uranium_atom_density.get::<
        uom::si::volumetric_number_density::per_cubic_centimeter>();

    dbg!(uranium_atom_density_per_cm3);


    // now let's get the macroscopic cross section 
    //
    // Sigma = n sigma
    let u235_macro_total_xs: LinearNumberDensity = 
        (uranium_atom_density * u235_total_xs).into();

    dbg!(u235_macro_total_xs);
    // now the thing is, we may want to use a type alias to define 
    // macroscopic cross section and microscopic cross section for 
    // ease of use 
    //
    // macroscopic cross section is in units of per meter 
    // and microscopic cross section is in units of meter sq

    pub type MacroscopicCrossSection = LinearNumberDensity;
    pub type MicroscopicCrossSection = Area;

    let u235_total_xs: MicroscopicCrossSection = 
        u235_abs_xs + u235_scatter_xs;
    let u235_macro_total_xs: MacroscopicCrossSection = 
        (uranium_atom_density * u235_total_xs).into();

    dbg!(&u235_macro_total_xs);

    // now that we've gotten this, here's the procedure,
    // take the particle with prevailing position and direction 
    // estimate its length given a random variable, and move the 
    // particle to its new location
    
    // first, let's sample how long it went
    //
    // the way to sample is to use the formula:
    //
    // l = - 1/Sigma_t * ln (xi) 
    //
    // where xi is a random variable from 0 to 1
    // 
    // let's get ln(xi) first

    
    // now, for random variables, I already copied over some code 
    // from OpenMC and translated it to Rust 

    let mut prn_seed: u64 = 1;
    // note that the distribution ranges from [0,1) where it does not 
    // include 1, hence I will use 1 as the upper bound and subtract 
    // a tiny amount
    let lower_bound = 0_f64;
    let upper_bound = 1_f64 - f64::EPSILON;
    let xi: f64 = uniform_distribution(lower_bound, upper_bound, &mut prn_seed).unwrap();

    // next, let's calculate ln(xi) 
    let lnxi = xi.ln();

    // now for the length 
    // recip is reciprocal (1/x)
    let mean_free_path: Length = u235_macro_total_xs.recip();
    let sampled_length: Length = - mean_free_path * lnxi;

    // now let's move particle_1 to a new location given its direction 

    // first, the direction 
    //
    // probably want to have some methods automating this

    let delta_x: Vec<Length> = particle_1.direction.iter().map(
        |unit_direction| {
            *unit_direction * sampled_length
        }
        ).collect();

    particle_1.position[0] += delta_x[0];
    particle_1.position[1] += delta_x[1];
    particle_1.position[2] += delta_x[2];

    // let's see the particle's position 
    dbg!(&particle_1.position);

    // next, I need to sample the reaction type.
    // First question is whether we scatter or absorb
    //
    // we coded earlier: 
    //
    // let u235_scatter_xs: Area = Area::new::<barn>(2.839);
    // let u235_total_xs: Area = u235_abs_xs + u235_scatter_xs;
    //
    let scatter_probability: Ratio = u235_scatter_xs/u235_total_xs;

    // the scatter probability is somewhere from [0,1)
    // so we'll need to take a random number from this range again with 
    // the same upper and lower bound as before
    let xi: f64 = uniform_distribution(lower_bound, upper_bound, &mut prn_seed).unwrap();

    // for readability, we may want to use an enum to denote this 

    #[derive(Debug)]
    pub enum NeutronInteraction {
        Scatter,
        Absorption
    }

    let mut interaction :NeutronInteraction;
    
    if xi > scatter_probability.value {
        // if it's more than scatter probability, we have absorption 
        // event, terminate
        interaction = NeutronInteraction::Absorption;
    } else {

        // else, we have more scattering
        interaction = NeutronInteraction::Scatter;
    }
    dbg!(&xi);
    dbg!(&interaction);

    // now what do we do if there is absorption?
    // we could do fission or parasitic absorption, but we don't bother 
    // at this point
    //
    // maybe I'll just have a count of how many absorptions there are. 
    // versus how many scatters
    let mut absorption_count: u64 = 0;
    let mut scatter_count: u64 = 0;

    match interaction {
        NeutronInteraction::Scatter => {
            scatter_count += 1;
        },
        NeutronInteraction::Absorption => {
            absorption_count += 1;
            // for absorption, probably want to delete the particle 
            // to free memory
            drop(particle_1);
        },
    }
    dbg!(&absorption_count);
    dbg!(&scatter_count);

    // now, if iteration is scattering only, then we need to sample a 
    // new direction for the particle
    //
    // in continuous energy mode, we also need to sample a new energy
    // but we shan't bother with it
    // 
    //
    // For this test, however, we didn't get a scattering interaction 
    // so I'll terminate the test here



    

    // basically, to print output, I use an explicit panic, turn off if 
    // not using
    let panic_debug: bool = false;

    if panic_debug {
        panic!();
    }



}

pub struct MockTestMonoenergeticParticle{
    pub position: [Length;3],
    pub direction: [Ratio;3],
}

impl Default for MockTestMonoenergeticParticle {
    fn default() -> Self {
        // position centered at zero 
        // direction in x direction, 

        let pos = [Length::ZERO;3];
        let dir = [ Ratio::new::<ratio>(1.0),
        Ratio::new::<ratio>(0.0),
        Ratio::new::<ratio>(0.0)];

        return Self {
            position: pos,
            direction: dir
        };
    }
}

