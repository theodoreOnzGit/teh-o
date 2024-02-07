use uom::si::angle::radian;
use uom::ConstZero;
use uom::si::f64::*;
use uom::si::ratio::ratio;
use teh_o::simulation::monte_carlo::openmc::random_dist::uniform_distribution;

pub type MacroscopicCrossSection = LinearNumberDensity;
pub type MicroscopicCrossSection = Area;
/// This test simulates one particle's random walk 
/// in an infinite medium with isotropic scattering and an 
/// absorption cross section
///
/// But with one guaranteed scattering reaction
#[test]
pub fn test2_random_walk_infinite_medium_scattering(){

    // now, based on test 1, we note that we will be using 
    // uniform distributions quite often to sample path lengths in 
    // the medium and to move it to a new spot
    //
    //
    use uom::si::area::barn;
    use uom::si::mass_density::gram_per_cubic_centimeter;
    use uom::si::molar_mass::gram_per_mole;

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
    // particle_1 to scatter
    let mut prn_seed: u64 = 5;
    // now, based on test 1, we note that we will be using 
    // uniform distributions quite often to sample path lengths in 
    // the medium and to move it to a new spot
    //
    // it's better in this case to encapsulate all these steps into a 
    // random walk function or random walk travel function
    particle_1.random_walk_travel(&mut prn_seed, u235_macro_total_xs);

    // let's see the particle's position 
    dbg!(&particle_1.position);

    let interaction = MockTestMonoenergeticParticle::
        scatter_or_absorption_rng(&mut prn_seed, u235_total_xs, u235_scatter_xs);

    dbg!(&interaction);

    // now what do we do if there is absorption?
    // we could do fission or parasitic absorption, but we don't bother 
    // at this point
    //
    // maybe I'll just have a count of how many absorptions there are. 
    // versus how many scatters
    //
    // for tally counting, I'll make a tally struct
    let mut collision_tally = CollisionTally::default();
    collision_tally.add_interaction_to_tally(interaction);

    // now, if iteration is scattering only, then we need to sample a 
    // new direction for the particle
    //
    // in continuous energy mode, we also need to sample a new energy
    // but we shan't bother with it
    //
    // Another simplification we make is that the particle is essentially 
    // at rest
    //
    // For this test, however, we didn't get a scattering interaction 
    // so I'll terminate the test here
    //
    // How shall we get a new direction??
    //
    // from the OpenMC docs, 
    // https://docs.openmc.org/en/stable/methods/neutron_physics.html#sample-angle
    //
    // We assume isotropic scattering for simplicity. In this case,
    // the scattering angle cosine is sampled using 

    let xi = MockTestMonoenergeticParticle::
        uniform_dist_zero_to_one_rng(&mut prn_seed);
    let mu_scatter: f64 = 2_f64 * xi - 1.0;

    // this should give a uniform distribution in [-1,1)
    // and then we can sample the azimuthal angle randomly as well
    //
    // for this, we can sample sine theta (theta is azimuthal angle) 
    // randomly from [-1,1) as well 

    let xi = MockTestMonoenergeticParticle::
        uniform_dist_zero_to_one_rng(&mut prn_seed);
    let sine_theta: f64 = 2_f64 * xi - 1.0;

    // now that we've gotten the angles, what next?
    //
    // first, we need to convert the coordinates to from cartesian to 
    // polar 
    //
    // next, we need to change the polar coordinates, 
    //
    // third, we need to convert the polar coordinates back to cartesian

    particle_1.obtain_new_direction_based_on_scatter_angle(
        mu_scatter, sine_theta);

    

    // basically, to print output, I use an explicit panic, turn off if 
    // not using
    let panic_and_debug: bool = true;

    if panic_and_debug {
        panic!();
    }



}


#[derive(Debug)]
pub enum NeutronInteraction {
    Scatter,
    Absorption
}

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
            return NeutronInteraction::Absorption;
        } else {

            // else, we have more scattering
            return NeutronInteraction::Scatter;
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

#[derive(Debug)]
pub struct CollisionTally {
    absorption_count: u64,
    scatter_count: u64,
}

impl Default for CollisionTally {
    /// start default with 0 counts each
    fn default() -> Self {
        Self { absorption_count: 0, scatter_count: 0 }
    }
}

impl CollisionTally {

    /// based on interaction type, add to tally 
    pub fn add_interaction_to_tally(&mut self,
        interaction: NeutronInteraction) {

        match interaction {
            NeutronInteraction::Scatter => {
                self.add_to_scatter_count();
            },
            NeutronInteraction::Absorption => {
                self.add_to_absorption_count();
            },
        }
    }

    /// add to absorption_count 
    #[inline]
    pub fn add_to_absorption_count(&mut self){
        self.absorption_count += 1;
    }
    /// add to scatter_count 
    #[inline]
    pub fn add_to_scatter_count(&mut self){
        self.scatter_count += 1;
    }

    /// reset absorption_count 
    pub fn reset_absorption_count(&mut self){
        self.absorption_count = 0;
    }
    /// reset scatter_count 
    pub fn reset_scatter_count(&mut self){
        self.scatter_count = 0;
    }

    /// get absorption count 
    pub fn get_absorption_count(&self) -> u64 {
        return self.absorption_count;
    }
    /// get scatter count 
    pub fn get_scatter_count(&self) -> u64 {
        return self.scatter_count;
    }
}


