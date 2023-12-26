use std::path::PathBuf;

#[derive(Debug,Default,PartialEq, PartialOrd, Clone)]
pub struct OpenMCSettings {
    /// assumes tallies are spatially separate
    pub assume_separate: Option<bool>,
    /// checks for overlaps in geometry 
    pub check_overlaps: Option<bool>,
    /// use confidence intervals for results?
    pub confidence_intervals: Option<bool>,
    /// create fission neutrons (fixed source) 
    pub create_fission_neutrons: Option<bool>,
    /// is a CMFD run?
    pub cmfd_run: Option<bool>,
    /// Scale fission photon yield to include delayed 
    pub delayed_photon_scaling: Option<bool>,

    /// calculate Shannon Entropy?
    pub entropy_on: Option<bool>,
    /// use event based mode rather than history based?
    pub event_based: Option<bool>,
    /// convert Legendre Distributions to Tabular?
    pub legendre_to_tabular: Option<bool>,
    /// create material cells offsets?
    pub material_cell_offsets: Option<bool>,
    /// write summary h5?
    pub output_summary : Option<bool>,
    /// write tallies.out?
    pub output_tallies : Option<bool>,
    /// particle restart run?
    pub particle_restart_run : Option<bool>,
    /// photon transport on?
    pub photon_transport : Option<bool>,
    /// reduce tallies at end of batch?
    pub reduce_tallies : Option<bool>,
    /// use resonance upscattering method?
    pub res_scat_on : Option<bool>,
    /// restart run?
    pub restart_run : Option<bool>,
    /// run with continuous energy data?
    pub run_continuous_energy : Option<bool>,
    /// write latest source at each batch?
    pub source_latest : Option<bool>,
    /// write source to separate file?
    pub source_separate : Option<bool>,
    /// write source to hdf5 file?
    pub source_write : Option<bool>,
    /// write source to mcpl file?
    pub source_mcpl_write : Option<bool>,
    /// write surface source file?
    pub surf_source_write : Option<bool>,
    /// write surface mcpl file?
    pub surf_mcpl_write : Option<bool>,

    /// use survival biasing?
    pub survival_biasing: Option<bool>,
    /// use multipole data?
    pub temperature_multipole: Option<bool>,
    /// tally triggers enabled?
    pub trigger_on: Option<bool>,
    /// predict batches for triggers?
    pub trigger_predict: Option<bool>,
    /// uniform fission site method on?
    pub uniform_fission_site_on: Option<bool>,
    /// use unresolved resonance probability tables?
    pub unresolved_res_ptables_on: Option<bool>,
    /// weight windows enabled?
    pub weight_windows_on: Option<bool>,
    /// Enable weight window check upon surface crossing?
    pub weight_window_checkpoint_surface: Option<bool>,
    /// Enable weight window check upon surface crossing?
    pub weight_window_checkpoint_collision: Option<bool>,
    /// write track files for every particle?
    pub write_all_tracks: Option<bool>,
    /// write out initial source file?
    pub write_initial_source: Option<bool>,

    // paths to various files
    /// path to cross_sections.xml
    pub path_cross_sections: Option<PathBuf>,
    /// path to input xml (may or may not use)
    pub path_input: Option<PathBuf>,
    /// path to output
    pub path_output: Option<PathBuf>,
    /// path to a particle restart file
    pub path_particle_restart: Option<PathBuf>,
    /// path to a source file
    pub path_sourcepoint: Option<PathBuf>,
    /// path to a statepoint file
    pub path_statepoint: Option<PathBuf>,
    /// Location of weight window file to 
    /// load of simulation initialisation
    pub weight_windows_file: Option<PathBuf>,


}
