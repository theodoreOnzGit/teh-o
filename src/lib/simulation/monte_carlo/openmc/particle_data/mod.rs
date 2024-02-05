use super::position::Position;
use super::position::Direction;


#[derive(Debug,Clone,Copy, PartialEq, PartialOrd)]
#[allow(non_snake_case)]
pub struct ParticleData {

    //  Position r_last_current_; //!< coordinates of the last collision or
    //                            //!< reflective/periodic surface crossing for
    //                            //!< current tallies
    //  Position r_last_;         //!< previous coordinates
    //  Direction u_last_;        //!< previous direction coordinates
    //  double wgt_last_ {1.0};   //!< pre-collision particle weight

    r_last_current_: Position,
    r_last_: Position, 
    u_last_: Direction,

    //  from openmc
    //  ParticleType& type() { return type_; }
    //  by default it is a neutron
    type_: ParticleType,

}

// in openmc: 
//
//
// enum class ParticleType { neutron, photon, electron, positron };
#[derive(Debug,Clone,Copy, PartialEq, PartialOrd)]
pub enum ParticleType {
    Neutron,
    Photon,
    Electron,
    Positron
}


// in ParticleData in openmc, these are defined...
//
// That's a WHOLE lot of data for each particle... I wonder if there 
// is an easier way to do this.
//public:
//  //----------------------------------------------------------------------------
//  // Constructors
//  ParticleData();
//
//private:
//  //==========================================================================
//  // Data members (accessor methods are below)
//
//  // Cross section caches
//  vector<NuclideMicroXS> neutron_xs_; //!< Microscopic neutron cross sections
//  vector<ElementMicroXS> photon_xs_;  //!< Microscopic photon cross sections
//  MacroXS macro_xs_;                  //!< Macroscopic cross sections
//  CacheDataMG mg_xs_cache_;           //!< Multigroup XS cache
//
//  int64_t id_;                                //!< Unique ID
//  ParticleType type_ {ParticleType::neutron}; //!< Particle type (n, p, e, etc.)
//
//  int n_coord_ {1};          //!< number of current coordinate levels
//  int cell_instance_;        //!< offset for distributed properties
//  vector<LocalCoord> coord_; //!< coordinates for all levels
//
//  // Particle coordinates before crossing a surface
//  int n_coord_last_ {1};  //!< number of current coordinates
//  vector<int> cell_last_; //!< coordinates for all levels
//
//  // Energy data
//  double E_;       //!< post-collision energy in eV
//  double E_last_;  //!< pre-collision energy in eV
//  int g_ {C_NONE}; //!< post-collision energy group (MG only)
//  int g_last_;     //!< pre-collision energy group (MG only)
//
//  // Other physical data
//  double wgt_ {1.0};       //!< particle weight
//  double mu_;              //!< angle of scatter
//  double time_ {0.0};      //!< time in [s]
//  double time_last_ {0.0}; //!< previous time in [s]
//
//  // Other physical data
//  Position r_last_current_; //!< coordinates of the last collision or
//                            //!< reflective/periodic surface crossing for
//                            //!< current tallies
//  Position r_last_;         //!< previous coordinates
//  Direction u_last_;        //!< previous direction coordinates
//  double wgt_last_ {1.0};   //!< pre-collision particle weight
//
//  // What event took place
//  bool fission_ {false};  //!< did particle cause implicit fission
//  TallyEvent event_;      //!< scatter, absorption
//  int event_nuclide_;     //!< index in nuclides array
//  int event_mt_;          //!< reaction MT
//  int delayed_group_ {0}; //!< delayed group
//
//  // Post-collision physical data
//  int n_bank_ {0};        //!< number of fission sites banked
//  int n_bank_second_ {0}; //!< number of secondary particles banked
//  double wgt_bank_ {0.0}; //!< weight of fission sites banked
//  int n_delayed_bank_[MAX_DELAYED_GROUPS]; //!< number of delayed fission
//                                           //!< sites banked
//
//  // Indices for various arrays
//  int surface_ {0};        //!< index for surface particle is on
//  int cell_born_ {-1};     //!< index for cell particle was born in
//  int material_ {-1};      //!< index for current material
//  int material_last_ {-1}; //!< index for last material
//
//  // Boundary information
//  BoundaryInfo boundary_;
//
//  // Temperature of current cell
//  double sqrtkT_ {-1.0};     //!< sqrt(k_Boltzmann * temperature) in eV
//  double sqrtkT_last_ {0.0}; //!< last temperature
//
//  // Statistical data
//  int n_collision_ {0}; //!< number of collisions
//
//  // Track output
//  bool write_track_ {false};
//
//  // Current PRNG state
//  uint64_t seeds_[N_STREAMS]; // current seeds
//  int stream_;                // current RNG stream
//
//  // Secondary particle bank
//  vector<SourceSite> secondary_bank_;
//
//  int64_t current_work_; // current work index
//
//  vector<double> flux_derivs_; // for derivatives for this particle
//
//  vector<FilterMatch> filter_matches_; // tally filter matches
//
//  vector<TrackStateHistory> tracks_; // tracks for outputting to file
//
//  vector<NuBank> nu_bank_; // bank of most recently fissioned particles
//
//  vector<double> pht_storage_; // interim pulse-height results
//
//  // Global tally accumulators
//  double keff_tally_absorption_ {0.0};
//  double keff_tally_collision_ {0.0};
//  double keff_tally_tracklength_ {0.0};
//  double keff_tally_leakage_ {0.0};
//
//  bool trace_ {false}; //!< flag to show debug information
//
//  double collision_distance_; // distance to particle's next closest collision
//
//  int n_event_ {0}; // number of events executed in this particle's history
//
//  // Weight window information
//  int n_split_ {0}; // Number of times this particle has been split
//  double ww_factor_ {
//    0.0}; // Particle-specific factor for on-the-fly weight window adjustment
//
//// DagMC state variables
//#ifdef DAGMC
//  moab::DagMC::RayHistory history_;
//  Direction last_dir_;
//#endif
//
//  int64_t n_progeny_ {0}; // Number of progeny produced by this particle
//
//public:
//  //==========================================================================
//  // Methods and accessors
//
//  NuclideMicroXS& neutron_xs(int i) { return neutron_xs_[i]; }
//  const NuclideMicroXS& neutron_xs(int i) const { return neutron_xs_[i]; }
//  ElementMicroXS& photon_xs(int i) { return photon_xs_[i]; }
//  MacroXS& macro_xs() { return macro_xs_; }
//  const MacroXS& macro_xs() const { return macro_xs_; }
//  CacheDataMG& mg_xs_cache() { return mg_xs_cache_; }
//  const CacheDataMG& mg_xs_cache() const { return mg_xs_cache_; }
//
//  int64_t& id() { return id_; }
//  const int64_t& id() const { return id_; }
//  ParticleType& type() { return type_; }
//  const ParticleType& type() const { return type_; }
//
//  int& n_coord() { return n_coord_; }
//  const int& n_coord() const { return n_coord_; }
//  int& cell_instance() { return cell_instance_; }
//  const int& cell_instance() const { return cell_instance_; }
//  LocalCoord& coord(int i) { return coord_[i]; }
//  const LocalCoord& coord(int i) const { return coord_[i]; }
//  const vector<LocalCoord>& coord() const { return coord_; }
//
//  LocalCoord& lowest_coord() { return coord_[n_coord_ - 1]; }
//  const LocalCoord& lowest_coord() const { return coord_[n_coord_ - 1]; }
//
//  int& n_coord_last() { return n_coord_last_; }
//  const int& n_coord_last() const { return n_coord_last_; }
//  int& cell_last(int i) { return cell_last_[i]; }
//  const int& cell_last(int i) const { return cell_last_[i]; }
//
//  double& E() { return E_; }
//  const double& E() const { return E_; }
//  double& E_last() { return E_last_; }
//  const double& E_last() const { return E_last_; }
//  int& g() { return g_; }
//  const int& g() const { return g_; }
//  int& g_last() { return g_last_; }
//  const int& g_last() const { return g_last_; }
//
//  double& wgt() { return wgt_; }
//  double wgt() const { return wgt_; }
//  double& mu() { return mu_; }
//  const double& mu() const { return mu_; }
//  double& time() { return time_; }
//  const double& time() const { return time_; }
//  double& time_last() { return time_last_; }
//  const double& time_last() const { return time_last_; }
//  bool alive() const { return wgt_ != 0.0; }
//
//  Position& r_last_current() { return r_last_current_; }
//  const Position& r_last_current() const { return r_last_current_; }
//  Position& r_last() { return r_last_; }
//  const Position& r_last() const { return r_last_; }
//  Position& u_last() { return u_last_; }
//  const Position& u_last() const { return u_last_; }
//  double& wgt_last() { return wgt_last_; }
//  const double& wgt_last() const { return wgt_last_; }
//
//  bool& fission() { return fission_; }
//  TallyEvent& event() { return event_; }
//  const TallyEvent& event() const { return event_; }
//  int& event_nuclide() { return event_nuclide_; }
//  const int& event_nuclide() const { return event_nuclide_; }
//  int& event_mt() { return event_mt_; }
//  int& delayed_group() { return delayed_group_; }
//
//  int& n_bank() { return n_bank_; }
//  int& n_bank_second() { return n_bank_second_; }
//  double& wgt_bank() { return wgt_bank_; }
//  int* n_delayed_bank() { return n_delayed_bank_; }
//  int& n_delayed_bank(int i) { return n_delayed_bank_[i]; }
//
//  int& surface() { return surface_; }
//  const int& surface() const { return surface_; }
//  int& cell_born() { return cell_born_; }
//  const int& cell_born() const { return cell_born_; }
//  int& material() { return material_; }
//  const int& material() const { return material_; }
//  int& material_last() { return material_last_; }
//  const int& material_last() const { return material_last_; }
//
//  BoundaryInfo& boundary() { return boundary_; }
//
//  double& sqrtkT() { return sqrtkT_; }
//  const double& sqrtkT() const { return sqrtkT_; }
//  double& sqrtkT_last() { return sqrtkT_last_; }
//
//  int& n_collision() { return n_collision_; }
//  const int& n_collision() const { return n_collision_; }
//
//  bool& write_track() { return write_track_; }
//  uint64_t& seeds(int i) { return seeds_[i]; }
//  uint64_t* seeds() { return seeds_; }
//  int& stream() { return stream_; }
//
//  SourceSite& secondary_bank(int i) { return secondary_bank_[i]; }
//  decltype(secondary_bank_)& secondary_bank() { return secondary_bank_; }
//  int64_t& current_work() { return current_work_; }
//  const int64_t& current_work() const { return current_work_; }
//  double& flux_derivs(int i) { return flux_derivs_[i]; }
//  const double& flux_derivs(int i) const { return flux_derivs_[i]; }
//  decltype(filter_matches_)& filter_matches() { return filter_matches_; }
//  FilterMatch& filter_matches(int i) { return filter_matches_[i]; }
//  decltype(tracks_)& tracks() { return tracks_; }
//  decltype(nu_bank_)& nu_bank() { return nu_bank_; }
//  NuBank& nu_bank(int i) { return nu_bank_[i]; }
//  vector<double>& pht_storage() { return pht_storage_; }
//
//  double& keff_tally_absorption() { return keff_tally_absorption_; }
//  double& keff_tally_collision() { return keff_tally_collision_; }
//  double& keff_tally_tracklength() { return keff_tally_tracklength_; }
//  double& keff_tally_leakage() { return keff_tally_leakage_; }
//
//  bool& trace() { return trace_; }
//  double& collision_distance() { return collision_distance_; }
//  int& n_event() { return n_event_; }
//
//  int n_split() const { return n_split_; }
//  int& n_split() { return n_split_; }
//
//  double ww_factor() const { return ww_factor_; }
//  double& ww_factor() { return ww_factor_; }
//
//#ifdef DAGMC
//  moab::DagMC::RayHistory& history() { return history_; }
//  Direction& last_dir() { return last_dir_; }
//#endif
//
//  int64_t& n_progeny() { return n_progeny_; }
//
//  // Accessors for position in global coordinates
//  Position& r() { return coord_[0].r; }
//  const Position& r() const { return coord_[0].r; }
//
//  // Accessors for position in local coordinates
//  Position& r_local() { return coord_[n_coord_ - 1].r; }
//  const Position& r_local() const { return coord_[n_coord_ - 1].r; }
//
//  // Accessors for direction in global coordinates
//  Direction& u() { return coord_[0].u; }
//  const Direction& u() const { return coord_[0].u; }
//
//  // Accessors for direction in local coordinates
//  Direction& u_local() { return coord_[n_coord_ - 1].u; }
//  const Direction& u_local() const { return coord_[n_coord_ - 1].u; }
//
//  //! Gets the pointer to the particle's current PRN seed
//  uint64_t* current_seed() { return seeds_ + stream_; }
//  const uint64_t* current_seed() const { return seeds_ + stream_; }
//
//  //! Force recalculation of neutron xs by setting last energy to zero
//  void invalidate_neutron_xs()
//  {
//    for (auto& micro : neutron_xs_)
//      micro.last_E = 0.0;
//  }
//
//  //! resets all coordinate levels for the particle
//  void clear()
//  {
//    for (auto& level : coord_)
//      level.reset();
//    n_coord_ = 1;
//  }
//
//  //! Get track information based on particle's current state
//  TrackState get_track_state() const;
//
//  void zero_delayed_bank()
//  {
//    for (int& n : n_delayed_bank_) {
//      n = 0;
//    }
//  }
//
//  void zero_flux_derivs()
//  {
//    for (double& d : flux_derivs_) {
//      d = 0;
//    }
//  }
