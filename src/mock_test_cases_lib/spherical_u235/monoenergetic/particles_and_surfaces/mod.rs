use uom::ConstZero;
use uom::si::f64::*;
use uom::si::ratio::ratio;
/// This test simulates one particle's random walk 
/// in an infinite medium with isotropic scattering and an 
/// absorption cross section
#[test]
pub fn test1_random_walk_infinite_medium(){

    use uom::si::area::barn;
    use uom::si::mass_density::gram_per_cubic_centimeter;
    use uom::si::amount_of_substance::mole;
    use uom::si::molar_mass::gram_per_mole;
    // first thing is that we want to simulate one single particle 
    // in an infinite medium with a fixed arbitrary 
    // macroscopic cross section fixed absorption cross section 
    //
    // we simulate the particle essentially until it is absorbed.
    
    // first things first, we need to simulate a particle with x,y,z 
    // coordinates as well as a unit vector indicating its direction
    // here it is: 

    let particle_1 = MockTestMonoenergeticParticle::default();

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


    // now let's get the macroscopic cross section 
    //
    // Sigma = n sigma
    let u235_macro_total_xs: LinearNumberDensity = 
        (uranium_atom_density * u235_total_xs).into();


    dbg!(&u235_macro_total_xs);


    panic!();



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
