
pub struct OpenMCSimulationEnvironment{
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
}
