use uom::si::area::barn;
use uom::ConstZero;
use uom::si::f64::*;
use uom::si::ratio::ratio;
/// This test simulates one particle's random walk 
/// in an infinite medium with isotropic scattering and an 
/// absorption cross section
#[test]
pub fn test1_random_walk_infinite_medium(){

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
