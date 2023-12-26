use super::timer::Timer;


#[derive(Debug,Default,PartialEq, PartialOrd, Clone)]
pub struct OpenMCSimulationEnvironmentVariables{
    current_batch: Option<u32>,
    current_gen: Option<u32>,
    initialised: bool,
    keff: f64,
    keff_std: Option<f64>,
    k_col_abs: f64,
    k_col_tra: f64,
    k_abs_tra: f64,
    log_spacing: Option<f64>,
    n_lost_particles: u32,
    need_depletion_rs: bool,
    restart_batch: Option<i32>,
    satisfy_triggers: bool,
    total_gen: u32,
    total_weight: Option<f64>,
    work_per_rank: Option<i64>,

    // RegularMesh and EntropyMesh 
    // need to be implemented
    k_generation: Option<Vec<f64>>,
    work_index: Option<Vec<i64>>,

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
}
