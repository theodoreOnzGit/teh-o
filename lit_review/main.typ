// set some latex settings 
#set par(leading: 0.55em, first-line-indent: 1.8em, justify: true)
#set text(font: "New Computer Modern")
#show raw: set text(font: "New Computer Modern Mono")
#show par: set block(spacing: 0.55em)
#show heading: set block(above: 1.4em, below: 1em)

#let numbered_eq(content) = math.equation(
    block: true,
    numbering: "(1)",
    content,
)

= Introduction 

In modelling pebble beds, whether in Fluoride salt cooled high temperature 
reactors (FHRs) or High Temperature Gas Cooled Reactors (HTGRs), we always 
run into the problem of double heterogeneity due to the presence of TRISO 
particles @Fratoni2008. This makes the time required for Monte Carlo 
simulation excessively long. While we may want to simplify the analysis 
of pebble beds via homogenisation methods, the pebbles and TRISO particles 
are small with respect to neutron diffusion lengths @Satvat2021. Therefore, 
neutron spectrum in each pebble is heavily influenced by that of adjacent 
pebbles. Therefore, unit cell models are not as suitable for neutronics 
analysis. The situation is further exacerbated by the fact that each adjacent 
pebble may have different composition and temperature. This would then 
further affect the neutron spectrum in adjacent pebbles, and therefore,
power distribution and burnup rates. Hence, homogenisation is not quite 
desirable for such cases. Therefore, there are requirements for 
methods to speed up Monte Carlo simulation in doubly heterogeneous systems. 

Now, of course, we can use deterministic methods to speed up simulation 
for reactor simulation. GeN-Foam uses this approach @Fiorina2017. 
However, we still need Monte Carlo methods in 
order to generate multi group cross sections (MGXS) for these simulations.
Additionally, to ensure that these simulations are reasonably accurate, we 
still need to benchmark these deterministic simulations against a 
high fidelity simulation. These high fidelity simulations would usually 
be Monte Carlo coupled directly to a thermal hydraulics solver such as 
Cardinal, which uses OpenMC Monte Carlo code coupled with the NekRS thermal 
hydraulics solver @Novak2022. This was used for a prismatic TRISO design 
which was not as doubly heterogeneous as a TRISO pebble bed.
Therefore, we still need to consider double heterogeneity in these cases.
Moreover, if we want to consider burnup and fuel depletion, then Monte 
Carlo simulation is essential. Let us consider some methods for speeding 
up Monte Carlo simulations for the purposes of MGXS generation.

For this work, any work on Method of Characteristics is out of scope.

= Homogenisation Methods 

While homogenisation is generally not desired for high fidelity, it is a 
good starting point in simulating an approximate representation of a 
doubly-heterogeneous pebble bed. This is where 
the double heterogeneity is eliminated from the pebble itself. Homogenisation 
is not preferred because we lose information about the TRISO geometry. 
Moreover, some of the spatial self shielding effects are lost when homogenising 
TRISO fuel with its graphite matrix @Fratoni2008. 

#let kinf = $k_infinity$

However, homogenisation is still helpful because it offers significant 
speed ups and makes the pebble bed simulation practical. Additionally,
some of the spatial self shielding effects can be preserved through 
two kinds of homogenisation. Firstly, Fratoni homogenised the TRISO shells 
with the graphite matrix, but left the TRISO fuel kernels intact @Fratoni2008.
This resulted in significant (25%) time saving compared to the original 
TRISO geometry while #kinf changed by about 0.2 % @Fratoni2008. A 
second class of methods is the reactivity-equivalent physical 
transform (RPT), where the TRISO particles are homogenised into 
a subregion of the pebble which could be a sphere or ring @Lou2020. The 
radius of the sphere or ring is adjusted such that the #kinf of the 
transformed pebble is equivalent to that of the original TRISO pebble.
This would partially account for some of the spatial self shielding 
effects, but would result in loss of information for the TRISO pebble.
This is most evident in depletion calculations involving burnable poisons 
and depletion. In the case of depletion calculations, RPT would have 
to be performed judiciously to ensure that the burnable posion 
concentrations deviate little from the fully doubly-heterogeneous
system. This has motivated the development of several iterations and 
improvements of the RPT method @Lou2021. While these methods are effective,
the RPT homogenisation methods still need to be benchmarked against a fully 
doubly-heterogeneous system to check if the depletion is accurate. If 
different burnable poisons and fuels are used, then we must check if the 
same variant of RPT method can work. This makes RPT an approximate method 
which could be used in only a subset of geometries where it has been verified 
and tested against a fully doubly-heterogeneous system. In this regard,
we still need a fully doubly-heterogeneous simulation in order to benchmark 
RPT.

= Delta Tracking 

Serpent uses a delta tracking methodology to speed up Monte Carlo simulations
@Leppaenen2010 based on rejection sampling @Leppaenen2015. This is used 
where neutron mean free path is long compared to the spatial dimensions 
of the geometry such as for the TRISO fuel in question. In delta tracking 
routines in Serpent and Serpent2, the neutrons can be moved to their 
next collision site without stopping the track at each boundary or surface 
@Leppaenen2015. For HTGR geometry, the speed up can be as high as 
10 times compared to traditional surface tracking @Leppaenen2015. However,
this method suffers from efficiency problems in regions with 
localised heavy absorbers @Leppaenen2015. Therefore, the algorithm switches 
to one based on surface tracking near these absorbers @Leppaenen2015. 
Thus, we can see that Delta Tracking is not a silver bullet.


Despite these drawbacks, Serpent has been successfully used for 
HTR-10 simulations when generating MGXS for 
the TORT-TD deterministic code @Setyawan2021. In simulating 
doubly-heterogeneous geometries, the advantages outweigh 
the drawbacks.

= Surface Tracking Speedup methods 

Distance caching was used in SCONE @Kowalski2021a, 
with a reported 7 to 20% speedup in simulation time @Kowalski2021.

= Energy Mesh Corasening and Multigroup Methods

Energy mesh coarsening was considered as well @Raffuzzi2023, because 
continuous energy spectrum for neutrons is computationally expensive.
For the inactive cycles, we could use multigroup cross sections rather 
than continuous cross sections for simulations so that we converge 
the fission source. After the fission source converges, the continuous 
group cross sections are used @Raffuzzi2023. This was explored in SCONE 
and was shown to accelerate calculations by 2.5 to 5 times. However it 
was only performed on inactive cycles @Raffuzzi2023. 
Therefore, the overall speed up was considerably less than 5 times.

= Spatial Mesh Coarsening Methods 

Kairos Power produced a generic FHR (gFHR) simulation based on separating 
the pebbles into spectral zones @Satvat2021 similar to what is done in VSOP.
The pebbles within each zone is assumed to experienced the same neutron 
spectrum and temperature, or more precisely, the deviations of neutron 
spectrum and temperature within each zone are small @Satvat2021. Therefore,
each spectral zone can be represented by one pebble, thus reducing the cost 
of simulation. This is essentially like mesh coarsening. 




= Computer Memory, GPU and CPU Optimisation 

The other class of methods involves optimisation of code to optimise memory 
usage, parallelisation and the use of the graphical processing unit (GPU).
This is done in the rewrite from Serpent to Serpent2, where the program 
was re-written to optimise memory usage, and to use OpenMP for parallelisation 
@Leppaenen2015.

For GPU computing, there were studies on using GPU for OpenMC as well 
@Tramm2022. The key change in algorithms is to go from history based 
transport for event based transport. In history based transport, for say 
10,000 particles, each thread is assigned a number of particles and it 
will trace the path of each particle to its finish. For event based transport,
particles are stored in a large buffer. Each particle is would not be 
traced to its completion, instead each particle would have its calculations 
carried out in stages (events). So for 10,000 particles, the 10,000 particles 
would all be traced to their next surface crossing. Their state would 
then be saved. Then when all particles have met their next surface, the
solver would move on to the next calculation stage or "event". This is 
supposed to improve the overall vector efficiency of the application
@Tramm2022.

In Tram's paper, other optimisation methods were introduced. For example,
it was found that looking up the cross sections was computationally expensive
@Tramm2022. One optimisation was that instead of referencing global variables,
one could reference a local variable on the stack @Tramm2022. 

Other optimisations included compiler optimisations for the low level 
virtual machine (LLVM) @Tramm2022 and algorithm optimisations for 
reducing memory access and calls. This could also be done for codes moving 
forward.


= Core Missions of Monte Carlo Simulations for doubly-heterogeneous geometry

== Burnup Calculations 

TBD

== MGXS Generation

In GeN-Foam, the SP3 equations look quite different @Fiorina2017:

#let genfoam_sp3_1 = $ 1/v_i (diff hat(phi.alt)_"0,i")/(diff t) &= nabla dot D_i nabla hat(phi.alt)_"0,i"
+ (nu Sigma_"f,i" (1- beta_t) chi_"p,i")/k_"eff" - Sigma_"r,i" hat(phi.alt)_"0,i" \
  &+ (S_"n,i" (1-beta_t) chi_"p,i")/k_"eff" + S_d chi_"d,i" + S_"s,i"
  + 2 Sigma_"r,i" phi.alt_"2,i" + 2 1/v_i (diff phi.alt_"2,i")/(diff t)  $

#let genfoam_sp3_2 = $ 3/v_i (diff phi.alt_"2,i")/(diff t) &= 3/7 
nabla dot 1/(Sigma_"t,i") nabla phi.alt_"2,i"
-(5/3 Sigma_"t,i" + 4/3 Sigma_"r,i") phi.alt_"2,i"
+ 2/3 Sigma_"r,i" hat(phi.alt)_"0,i" \
 &- 2/3 (nu Sigma_"f,i" (1- beta_t) chi_"p,i")/k_"eff" hat(phi.alt)_"0,i" 
  - 2/3 (S_"n,i" (1-beta_t) chi_"p,i")/k_"eff" - 2/3 S_d chi_"d,i" 
  - 2/3 S_"s,i"
  + 2/3 1/v_i (diff hat(phi.alt)_"0,i")/(diff t) \ $

#genfoam_sp3_1 
#genfoam_sp3_2

Where the fission sources, delayed neutron and scattering 
sources between energy groups are defined:

$ S_"n,i" = sum_(j eq.not i) nu Sigma_(j,i) phi.alt_(0,j) $
$ S_"d" = sum_k lambda_k C_k $
$ S_"s,i" = sum_(j eq.not i) Sigma_(j arrow.r i) phi.alt_(0,j) $

The tricky thing here is to obtain removal cross sections. 

== Transient Analysis

TBD

= Honourable Mentions

Geant4 was used to model HTGRs as well @Cilliers2022, however, I could find 
no novel method for speeding up simulations.



#bibliography("main.bib",
style: "chicago-author-date")

