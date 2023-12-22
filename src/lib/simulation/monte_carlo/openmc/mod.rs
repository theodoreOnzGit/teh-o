use self::timer::Timer;

pub mod timer;

// replacement for the namespace "simulation" in C
#[derive(Debug,Default,PartialEq, PartialOrd, Clone, Copy)]
pub struct OpenMCSimulationData { 
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
