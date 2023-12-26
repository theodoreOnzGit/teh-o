use self::openmc_settings::OpenMCSettings;

use super::timer::Timer;


pub mod openmc_settings;

// replacement for the namespace "simulation" in C
#[derive(Debug,Default,PartialEq, Clone)]
pub struct OpenMCSimulationEnvironmentVariables{
    pub current_batch: Option<u32>,
    pub current_gen: Option<u32>,
    pub initialised: bool,
    pub keff: f64,
    pub keff_std: Option<f64>,
    pub k_col_abs: f64,
    pub k_col_tra: f64,
    pub k_abs_tra: f64,
    pub log_spacing: Option<f64>,
    pub n_lost_particles: u32,
    pub need_depletion_rs: bool,
    pub restart_batch: Option<i32>,
    pub satisfy_triggers: bool,
    pub total_gen: u32,
    pub total_weight: Option<f64>,
    pub work_per_rank: Option<i64>,

    // RegularMesh and EntropyMesh 
    // need to be implemented
    pub k_generation: Option<Vec<f64>>,
    pub work_index: Option<Vec<i64>>,

    pub time_active: Timer,
    pub time_bank: Timer,
    pub time_bank_sample: Timer,
    pub time_bank_sencrcv: Timer,

    pub time_finalise: Timer,
    pub time_inactive: Timer,
    pub time_initialise: Timer,
    pub time_read_xs: Timer,
    pub time_statepoint: Timer,
    pub time_tallies: Timer,
    pub time_total: Timer,
    pub time_transport: Timer,
    pub time_event_init: Timer,
    pub time_event_calculate_xs: Timer,
    pub time_event_advance_particle: Timer,
    pub time_event_surface_crossing: Timer,
    pub time_event_collision: Timer,
    pub time_event_death: Timer,

    /// openmc settings 
    pub settings: OpenMCSettings,
}
