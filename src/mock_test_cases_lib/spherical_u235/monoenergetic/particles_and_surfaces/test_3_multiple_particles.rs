use uom::si::angle::radian;
use uom::ConstZero;
use uom::si::f64::*;
use uom::si::ratio::ratio;
use teh_o::simulation::monte_carlo::openmc::random_dist::uniform_distribution;


pub type MacroscopicCrossSection = LinearNumberDensity;
pub type MicroscopicCrossSection = Area;
/// This test simulates many particles' random walk 
/// in an infinite medium with isotropic scattering and an 
/// absorption cross section
///
/// But with one guaranteed scattering reaction
#[test]
pub fn test3_random_walk_infinite_medium_scattering_many_particle(){

    // okay, usual cross sections stuff
    use uom::si::area::barn;
    use uom::si::mass_density::gram_per_cubic_centimeter;
    use uom::si::molar_mass::gram_per_mole;
    use approx::assert_relative_eq;

    let mut particle_1 = MockTestMonoenergeticParticle::default();

    // For cross sections:
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
    dbg!(&u235_scatter_xs);

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


    let u235_total_xs: MicroscopicCrossSection = 
        u235_abs_xs + u235_scatter_xs;
    let u235_macro_total_xs: MacroscopicCrossSection = 
        (uranium_atom_density * u235_total_xs).into();

    // we let the prn_seed be 5 now, so that it should cause the 
    // particle_1 to scatter first
    // set the tally and interaction
    let mut collision_tally = CollisionTally::default();
    let mut interaction = NeutronInteraction::_Scatter;

    // pseudorandom number generator seed
    let mut prn_seed: u64 = 5;

    // for each particle, this loop needs to be performed

    while interaction == NeutronInteraction::_Scatter {

        // if scattering, then we sample new distance 
        particle_1.random_walk_travel(&mut prn_seed, u235_macro_total_xs);

        // check the interaction there
        interaction = MockTestMonoenergeticParticle::
            scatter_or_absorption_rng(&mut prn_seed, u235_total_xs, u235_scatter_xs);

        // score the collision tally 
        collision_tally._add_interaction_to_tally(interaction);

        if interaction == NeutronInteraction::_Scatter {
            // get new direction if it is scattering 
            // but I'll abstract this into a isotropic scattering function



            particle_1.isotropically_scatter(
                &mut prn_seed);
        }


        if interaction == NeutronInteraction::_Absorption {
            // print particle 1 statistics, and delete it
            //dbg!(particle_1);
            //dbg!(collision_tally);
            break;
        }
    }

    // let's repeat for 1000 particles
    let mut collision_tally = CollisionTally::default();
    let mut interaction = NeutronInteraction::_Scatter;
    let neutron = MockTestMonoenergeticParticle::default();
    
    // I'll clone this 1000 times
    
    let mut neutron_vector = vec![neutron;1000];

    // I'll then perform monte carlo simulations in series first
    // This can be a map function 
    //
    dbg!(&neutron_vector.len());

    for neutron_ref in neutron_vector.iter_mut(){
        while interaction == NeutronInteraction::_Scatter {

            // if scattering, then we sample new distance 
            neutron_ref.random_walk_travel(&mut prn_seed, u235_macro_total_xs);

            // check the interaction there
            interaction = MockTestMonoenergeticParticle::
                scatter_or_absorption_rng(&mut prn_seed, u235_total_xs, u235_scatter_xs);

            // score the collision tally 
            collision_tally._add_interaction_to_tally(interaction);

            if interaction == NeutronInteraction::_Scatter {
                // get new direction if it is scattering 
                // but I'll abstract this into a isotropic scattering function
                neutron_ref.isotropically_scatter(
                    &mut prn_seed);
            } else if interaction == NeutronInteraction::_Absorption {
                // print particle 1 statistics, and delete it
                //dbg!(&neutron_ref);
                //dbg!(&collision_tally);
            }
        }
        // need to reset interaction to scatter, 
        // because after neutron is absorbed, the interaction is absorption 
        // and then , the loop 
        // won't follow through
        interaction = NeutronInteraction::_Scatter;
    }

    // after this, we are done with the collision tally,
    // let's read the samples

    let absorption_count = collision_tally._absorption_count;
    let scatter_count = collision_tally._scatter_count;

    // now the scatter to absorption_count is quite important to note 
    // ideally, we should get the statistics equal, or at least similar to their 
    // cross section ratios
    //

    dbg!(collision_tally);

    let scatter_to_absorption_monte_carlo: f64 = (scatter_count as f64)/
        (absorption_count as f64);

    let scatter_to_absorption_cross_sections: f64 = 
        u235_scatter_xs.value / u235_abs_xs.value;

    // make sure that these agree to within 0.5%
    assert_relative_eq!(
        scatter_to_absorption_cross_sections,
        scatter_to_absorption_monte_carlo,
        max_relative=0.005);

    // basically, to print output, I use an explicit panic, turn off if 
    // not false
    let panic_and_debug: bool = false;

    if panic_and_debug {
        panic!();
    }



}


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum NeutronInteraction {
    _Scatter,
    _Absorption
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct MockTestMonoenergeticParticle{
    pub position: [Length;3],
    pub direction: [Ratio;3],
}

impl MockTestMonoenergeticParticle {
    pub fn random_walk_travel(&mut self, 
        prn_seed: &mut u64,
        u235_macro_total_xs: MacroscopicCrossSection) {
        let lower_bound = 0_f64;
        let upper_bound = 1_f64 - f64::EPSILON;
        let xi: f64 = uniform_distribution(lower_bound, upper_bound, prn_seed).unwrap();

        // next, let's calculate ln(xi) 
        let lnxi = xi.ln();

        // now for the length 
        // recip is reciprocal (1/x)
        let mean_free_path: Length = u235_macro_total_xs.recip();
        let sampled_length: Length = - mean_free_path * lnxi;

        // now let's move this particle to a new location given its direction 

        // first, the direction 
        //
        // probably want to have some methods automating this

        let delta_x: Vec<Length> = self.direction.iter().map(
            |unit_direction| {
                *unit_direction * sampled_length
            }
        ).collect();

        self.position[0] += delta_x[0];
        self.position[1] += delta_x[1];
        self.position[2] += delta_x[2];

    }

    /// for convenience, I put the uniform distribution from 0 to 1 here
    #[inline]
    pub fn uniform_dist_zero_to_one_rng(prn_seed: &mut u64) -> f64 {
        let lower_bound = 0_f64;
        let upper_bound = 1_f64 - f64::EPSILON;
        let xi: f64 = uniform_distribution(lower_bound, upper_bound, prn_seed).unwrap();

        return xi;

    }

    /// for convenience, I also put the rng for scatter or absorption 
    /// here 
    #[inline] 
    pub fn scatter_or_absorption_rng(prn_seed: &mut u64,
        u235_total_xs: MicroscopicCrossSection,
        u235_scatter_xs: MicroscopicCrossSection) -> NeutronInteraction {

        let scatter_probability: Ratio = u235_scatter_xs/u235_total_xs;

        // the scatter probability is somewhere from [0,1)
        // so we'll need to take a random number from this range again with 
        // the same upper and lower bound as before
        let xi = MockTestMonoenergeticParticle::uniform_dist_zero_to_one_rng(
            prn_seed);

        if xi > scatter_probability.value {
            // if it's more than scatter probability, we have absorption 
            // event, terminate
            return NeutronInteraction::_Absorption;
        } else {

            // else, we have more scattering
            return NeutronInteraction::_Scatter;
        }

    }

    // Obtains a new direction based on the scattering solid angle 
    // based on mu_scatter, the cosine of the polar angle
    // sine_theta, the sine of the azimuthal angle
    #[inline] 
    pub fn obtain_new_direction_based_on_scatter_angle(&mut self,
        mu_scatter: f64,
        sine_theta: f64){

        // now let's determine the polar angle first,
        // https://math.libretexts.org/Courses/Monroe_Community_College/MTH_212_Calculus_III/Chapter_11%3A_Vectors_and_the_Geometry_of_Space/11.7%3A_Cylindrical_and_Spherical_Coordinates
        let x = self.position[0];
        let y = self.position[1];
        let z = self.position[2];

        let rho_sq: Area = x * x + y * y + z * z;

        let mu_particle: Ratio = z * rho_sq.sqrt().recip();
        let tan_theta_particle: Ratio = y/x;

        let polar_angle_particle = Angle::new::<radian>(
            mu_particle.get::<ratio>().acos());
        let azimuthal_angle_particle = Angle::new::<radian>(
            tan_theta_particle.get::<ratio>().atan());

        // now we can apply the two angles 
        let polar_angle_scatter = Angle::new::<radian>(
            mu_scatter.acos());
        let azimuthal_angle_scatter = Angle::new::<radian>(
            sine_theta.asin());

        // the logic here needs to be considered, especially that for 
        // the azimuthal angle since it ranges from 0 to 360
        // TODO, check this
        let new_polar_angle = polar_angle_particle + polar_angle_scatter;
        let new_azimuthal_angle = azimuthal_angle_particle + 
            2.0 * azimuthal_angle_scatter;

        // now that we have the new angles, we can set the new directions

        let sine_phi: Ratio = new_polar_angle.sin();
        let cos_phi: Ratio = new_polar_angle.cos();
        let sine_theta: Ratio = new_azimuthal_angle.sin();
        let cos_theta: Ratio = new_azimuthal_angle.cos();

        let x_new = sine_phi * cos_theta;
        let y_new = sine_phi * sine_theta;
        let z_new = cos_phi;

        self.direction = [x_new,y_new,z_new];

    }

    /// isotropic scattering 
    #[inline]
    pub fn isotropically_scatter(&mut self, prn_seed: &mut u64){

        let xi = MockTestMonoenergeticParticle::
            uniform_dist_zero_to_one_rng(prn_seed);
        let mu_scatter: f64 = 2_f64 * xi - 1.0;

        let xi = MockTestMonoenergeticParticle::
            uniform_dist_zero_to_one_rng(prn_seed);
        let sine_theta: f64 = 2_f64 * xi - 1.0;


        self.obtain_new_direction_based_on_scatter_angle(
            mu_scatter, sine_theta);
    }
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

#[derive(Debug,Clone, Copy)]
pub struct CollisionTally {
    _absorption_count: u64,
    _scatter_count: u64,
}

impl Default for CollisionTally {
    /// start default with 0 counts each
    fn default() -> Self {
        Self { _absorption_count: 0, _scatter_count: 0 }
    }
}

impl CollisionTally {

    /// based on interaction type, add to tally 
    pub fn _add_interaction_to_tally(&mut self,
        interaction: NeutronInteraction) {

        match interaction {
            NeutronInteraction::_Scatter => {
                self._add_to_scatter_count();
            },
            NeutronInteraction::_Absorption => {
                self._add_to_absorption_count();
            },
        }
    }

    /// add to absorption_count 
    #[inline]
    pub fn _add_to_absorption_count(&mut self){
        self._absorption_count += 1;
    }
    /// add to scatter_count 
    #[inline]
    pub fn _add_to_scatter_count(&mut self){
        self._scatter_count += 1;
    }

    /// reset absorption_count 
    pub fn _reset_absorption_count(&mut self){
        self._absorption_count = 0;
    }
    /// reset scatter_count 
    pub fn _reset_scatter_count(&mut self){
        self._scatter_count = 0;
    }

    /// get absorption count 
    pub fn _get_absorption_count(&self) -> u64 {
        return self._absorption_count;
    }
    /// get scatter count 
    pub fn _get_scatter_count(&self) -> u64 {
        return self._scatter_count;
    }
}


