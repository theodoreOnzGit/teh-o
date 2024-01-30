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
simulation excessively long. Therefore, there are requirements for 
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





= MGXS Generation

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

== Missing Higher Order Anisotropic Scattering Terms from GeN-Foam's version of SP3

Secondly, I had 
a doubt about the SP3 equations because they contained higher moments of 
scattering cross section usually denoted $Sigma_"s2"$ and $Sigma_"s3"$.
How are these going to be in the OpenMC tallies?

In the original SP3 equations @Larsen1996:

#let sp3_eqn_1 = $ - nabla (1/(3 Sigma_"a1")) nabla (phi.alt_0 + 2 phi.alt_2) + 
Sigma_"a0" phi.alt_0 = Q $

#let sp3_eqn_2 = $ - nabla (9/(35 Sigma_"a3")) nabla phi.alt_2 + 
Sigma_"a2" phi.alt_2 = 2/5 (Sigma_"a0" phi.alt_0 - Q) $

#sp3_eqn_1
#sp3_eqn_2

In this case, $Sigma_"a1" = Sigma_t - Sigma_"s1"$ and 
$Sigma_"a3" = Sigma_t - Sigma_"s3"$. $Sigma_"al"$ is also commonly 
denoted as $Sigma_"tr"$. Somehow, these terms are absent 
from the GeN-Foam equations. How?

Remember, the higher moment scattering equations come from Legendre 
polynomial expansion of the scattering cross section where we neglect 
azimuthal scattering effects. This can be written as:

#numbered_eq(
  $ Sigma_"s" (z, mu_0) = sum_(l=0)^infinity (2l + 1)/(4 pi) 
  Sigma_"sl" P_l (mu_0) $
)<eqn:scattering-xs>

The SP3 equations were derived in an ad-hoc manner from the P3 equations 
for a slab geometry @Larsen1996. Hence only one coordinate (z) is considered.
For a slab geometry, the scattering cross section for P3 equations can 
be expressed as:

$ Sigma_"s" (z, mu_0) = 1/(4 pi) Sigma_"s0" P_0 (mu_0) 
+ 3/(4 pi) Sigma_"s1" P_1 (mu_0) 
+ 5/(4 pi) Sigma_"s2" P_2 (mu_0) 
+ 7/(4 pi) Sigma_"s3" P_3 (mu_0) $

Taking advantage of orthogonality:

$ 2 pi integral_(-1)^1 d mu P_"l1" (mu) P_"l2" (mu) := cases(
  (2l + 1)/(4 pi) "if" "l1"="l2",
  0 "if" "l1"eq.not"l2"
) $

So in general, if we consider $Sigma_s$ as defined in 
#ref(<eqn:scattering-xs>):

$ Sigma_"sl" = 2 pi integral_(-1)^1 d mu P_l(mu) Sigma_s (mu) $

Where $l$ is 0, 1, 2 and 3 for the different scattering coefficients.

For reference, the Legendre polynomials are:

$ P_l (x) = 1/(2^l l!) (d^l)/(d x^l) (x^2 - 1)^l $

For $l$ is 0, 1, 2 and 3:

$ P_0 (x) = 1/(2^0 0!) (d^0)/(d x^0) (x^2 - 1)^0 = 1 $
$ P_1 (x) = 1/(2^1 1!) (d^1)/(d x^1) (x^2 - 1)^1 = x $
$ P_2 (x) = 1/(2^2 2!) (d^2)/(d x^2) (x^2 - 1)^2 = 3/2 x^2 - 1/2 $
$ P_3 (x) = 1/(2^3 3!) (d^3)/(d x^3) (x^2 - 1)^3 = 
1/8 (35 x^4 - 30 x^2 + 3) $



Let's go back to the SP3 equations by Larsen @Larsen1996:


#sp3_eqn_1
#sp3_eqn_2

Suppose now that we had linearly anisotropic scattering cross sections.
We can reduce:

$ Sigma_"s" (z, mu_0) = 1/(4 pi) Sigma_"s0" P_0 (mu_0) 
+ 3/(4 pi) Sigma_"s1" P_1 (mu_0) 
+ 5/(4 pi) Sigma_"s2" P_2 (mu_0) 
+ 7/(4 pi) Sigma_"s3" P_3 (mu_0) $

To:

$ Sigma_"s" (z, mu_0) = 1/(4 pi) Sigma_"s0" P_0 (mu_0) 
+ 3/(4 pi) Sigma_"s1" P_1 (mu_0) $

In other words, $Sigma_"s2" = 0$ and $Sigma_"s3" = 0$. This would represent 
a linearly anisotropic scattering cross section. We call this linear because 
$P_0 (mu_0) = 1$ and $P_1 (mu_0) = mu_0$.

$ Sigma_"s" (z, mu_0) = 1/(4 pi) Sigma_"s0" + 3/(4 pi) Sigma_"s1" mu_0 $

With this simplification, $Sigma_"a2" = Sigma_t$ and $Sigma_"a3" = Sigma_t$.

#let sp3_eqn_1 = $ - nabla (1/(3 Sigma_"a1")) nabla (phi.alt_0 + 2 phi.alt_2) + 
Sigma_"a0" phi.alt_0 = Q $

#let sp3_eqn_2 = $ - nabla (9/(35 Sigma_"t")) nabla phi.alt_2 + 
Sigma_"t" phi.alt_2 = 2/5 (Sigma_"a0" phi.alt_0 - Q) $

#sp3_eqn_1
#sp3_eqn_2

Let's change the variables $hat(phi.alt)_0 = phi.alt_0 + 2 phi.alt_2$:

#let sp3_eqn_1 = $ - nabla (1/(3 Sigma_"a1")) nabla hat(phi.alt)_0 + 
Sigma_"a0" (hat(phi.alt)_0 - 2 phi.alt_2) = Q $

#sp3_eqn_1

And for the second equation:
#sp3_eqn_2

#let sp3_eqn_2 = $ - nabla (9/(35 Sigma_"t")) nabla phi.alt_2 + 
Sigma_"t" phi.alt_2 = 2/5 (Sigma_"a0" hat(phi.alt)_0 - 
2 Sigma_"a0" phi.alt_2 - Q) $

//#sp3_eqn_2
#let sp3_eqn_2 = $ - nabla (9/(35 Sigma_"t")) nabla phi.alt_2 + 
Sigma_"t" phi.alt_2 + 4/5 Sigma_t phi.alt_2 = 
2/5 (Sigma_"a0" hat(phi.alt)_0  - Q) $
//#sp3_eqn_1
#sp3_eqn_2




Now, let's compare this with the GeN-Foam equations:

Under steady state:

#let genfoam_sp3_1 = $ 0 &= nabla dot D_i nabla hat(phi.alt)_"0,i"
+ (nu Sigma_"f,i" (1- beta_t) chi_"p,i")/k_"eff" - Sigma_"r,i" hat(phi.alt)_"0,i" \
  &+ (S_"n,i" (1-beta_t) chi_"p,i")/k_"eff" + S_d chi_"d,i" + S_"s,i"
  + 2 Sigma_"r,i" phi.alt_"2,i" $

#let genfoam_sp3_2 = $ 0 &= 3/7 
nabla dot 1/(Sigma_"t,i") nabla phi.alt_"2,i"
-(5/3 Sigma_"t,i" + 4/3 Sigma_"r,i") phi.alt_"2,i"
+ 2/3 Sigma_"r,i" hat(phi.alt)_"0,i" \
 &- 2/3 (nu Sigma_"f,i" (1- beta_t) chi_"p,i")/k_"eff" hat(phi.alt)_"0,i" 
  - 2/3 (S_"n,i" (1-beta_t) chi_"p,i")/k_"eff" - 2/3 S_d chi_"d,i" 
  - 2/3 S_"s,i"
  $

#genfoam_sp3_1 
#genfoam_sp3_2

One group assumption $Sigma_"r,i" = Sigma_"abs"$:

#let genfoam_sp3_1 = $ 0 = nabla dot D nabla hat(phi.alt)_"0"
+ (nu Sigma_"f" (1- beta_t) chi_"p")/k_"eff" - Sigma_"abs" hat(phi.alt)_"0" 
  + S_d chi_"d"  
  + 2 Sigma_"abs" phi.alt_"2" $

#let genfoam_sp3_2 = $ 0 &= 3/7 
nabla dot 1/(Sigma_"t") nabla phi.alt_"2"
-(5/3 Sigma_"t" + 4/3 Sigma_"abs") phi.alt_"2"
+ 2/3 Sigma_"abs" hat(phi.alt)_"0" \
 &- 2/3 (nu Sigma_"f" (1- beta_t) chi_"p")/k_"eff" hat(phi.alt)_"0" 
  - 2/3 S_d chi_"d" 
  $


#genfoam_sp3_1 
#genfoam_sp3_2

Just to make the equations look similar, let us substitute the source 
as the multiplying media:

$ Q = (nu Sigma_"f" (1- beta_t) chi_"p")/k_"eff" hat(phi.alt)_"0" 
+ S_d chi_d $

Of course, this isn't quite that accurate per se as the source itself is 
dependent on $hat(phi.alt)$, but I'm only making the equations 
look similar:

#let genfoam_sp3_1 = $ 0 = nabla dot D nabla hat(phi.alt)_"0"
+ Q - Sigma_"abs" hat(phi.alt)_"0" 
+ 2 Sigma_"abs" phi.alt_"2" $


#let genfoam_sp3_1 = $ -nabla dot D nabla hat(phi.alt)_"0" = 
 Q - Sigma_"abs" hat(phi.alt)_"0" 
+ 2 Sigma_"abs" phi.alt_"2" $

#let genfoam_sp3_2 = $ 0 &= 3/7 
nabla dot 1/(Sigma_"t") nabla phi.alt_"2"
-(5/3 Sigma_"t" + 4/3 Sigma_"abs") phi.alt_"2"
+ 2/3 Sigma_"abs" hat(phi.alt)_"0" - 2/3 Q  $

#genfoam_sp3_1 
#genfoam_sp3_2

Now, the first equation looks more or less similar:

#sp3_eqn_1
#genfoam_sp3_1

The second equation needs more simplifying. Just multiply by 3/5:

#let genfoam_sp3_2 = $ 0 &= 3/7 
nabla dot 1/(Sigma_"t") nabla phi.alt_"2"
-(5/3 Sigma_"t" + 4/3 Sigma_"abs") phi.alt_"2"
+ 2/3 Sigma_"abs" hat(phi.alt)_"0" - 2/3 Q  $

#let genfoam_sp3_2 = $ 0 &= 9/35 
nabla dot 1/(Sigma_"t") nabla phi.alt_"2"
-(Sigma_"t" + 4/5 Sigma_"abs") phi.alt_"2"
+ 2/5 Sigma_"abs" hat(phi.alt)_"0" - 2/5 Q  $

For the GeN-Foam equation:
#genfoam_sp3_2

#let genfoam_sp3_2 = $ - 9/35 
nabla dot 1/(Sigma_"t") nabla phi.alt_"2"
+(Sigma_"t" + 4/5 Sigma_"abs") phi.alt_"2"
 &=  2/5 Sigma_"abs" hat(phi.alt)_"0" - 2/5 Q  $

#genfoam_sp3_2

Compare this to the assumption of linearly anisotropic scattering:

#sp3_eqn_2

If we can accept that
$Q = (nu Sigma_"f" (1- beta_t) chi_"p")/k_"eff" hat(phi.alt)_"0" + S_d chi_d$,
and that $D = (1/(3 Sigma_"a1"))$,

Then the form of SP3 equations in GeN-Foam would be identical to that 
of the second order sp3 equations in Larsen's work under the simplification 
of linearly anisotropic scattering cross section.



#bibliography("main.bib",
style: "chicago-author-date")

