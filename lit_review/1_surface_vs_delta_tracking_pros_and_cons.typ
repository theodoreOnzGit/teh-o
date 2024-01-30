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

Serpent's major speedups, especially for pebble bed geometries, 
are a result of its delta-tracking algorithms @Leppaenen2017. Here, we 
wish to gain some intuition about delta-tracking vs surface-tracking 
algorithms.


=  Homogenous Medium

Suppose we have a neutron going with a single energy E, such that 
the total macroscopic cross section is $Sigma_t$. The probability 
of collision at length $l$ is @Leppaenen2017:

#numbered_eq(
$ p(l) = Sigma_t exp(-l Sigma_t) $
)<eqn:prob-length-travelled>

#let mfp = $1/Sigma_t$

Where the mean free path $l_"mfp"$ is #mfp. Without showing the integral here, we 
can estimate the length travelled using @Leppaenen2017:

#let length_sample = $ l = - (ln xi)/Sigma_t = - l_"mfp" ln xi $

#length_sample

Where $xi$ is a random variable within the interval of 0 to 1, such that 
$ln xi < 0$. 

Basically, integrate #ref(<eqn:prob-length-travelled>) with respect 
to length from 0 to some length l (I have to check this) in order to 
get the probability of going to that length. For a homogeneous medium,
we can assume that $Sigma_t$ is constant through the neutron's path.

Should a particle collide in this medium, we then need to sample whether 
a fission or absorption happens.


= Heterogenous Medium

== Surface Tracking

Unfortunately, HTGRs and FHRs are not at all homogeneous. There will be 
many different materials with different $Sigma_t$. At each surface, 
we need to verify where the particle is and where it is going, 
and then change $Sigma_t$ according to the cell it is in whenever we meet 
a surface.

#image("surface-tracking.jpg")

The fact we have to track when we meet surfaces is problematic in a system 
with several million surfaces such as TRISO geometry for pebble beds. 
This is computationally expensive. This is the traditional 
surface tracking method.

== Nondimensionalised surface-tracking

Now, we can save on some of the computation cost by estimating how many 
mean free paths a particle will travel before collision. This was mentioned 
in literature for Serpent, but not implemented in Serpent @Leppaenen2017:

Recall:

#length_sample

Suppose now that we nondimensionalised the equation in terms of mean free 
paths:

$ l/l_"mfp" = - ln xi $

#let nd_mfp = $l/l_"mfp"$

We can calculate #nd_mfp once and only once. Thus, we save on the 
computational expense of calculating $ln xi$ and the random number 
generator. Whenever we travel in a medium with some finite mean free path, 
we calculate #nd_mfp accumulated in that medium and subtract it from the 
overall #nd_mfp calculated from the particle until it reaches zero.

Nevertheless, we still need to calculate $Sigma_t$ for each cell based on 
the energy of the particle. This search is still computationally expensive 
@Tramm2022 especially since we may have to perform it every time we cross 
a boundary.

== Delta Tracking

Now, suppose we could homogenise the cross sections so that we don't 
care if the particle crosses the surface or not. For this case, 
let's say for example that $Sigma_"t,fuel" > Sigma_"t,graphite"$. How 
shall we then homogenise the cross sections?

If you think about it, there are at least 100 kinds of reactions between 
the neutrons and the nucleus (eg $Sigma_s$, $Sigma_f$,
$Sigma_"n,2n"$ etc). If we were to use them individually to sample 
the collision length, that would be quite inefficient. We sort of added them 
up to a homogenised $Sigma_t$ for the region of fuel.

Should we have a collision we would roll the dice to see what reaction we 
actually ended up with.

#image("sigma-total-and-deciding-rxns.jpg")

What if we could use this system to homogenise the medium between the fuel 
and graphite?

Perhaps, we could try:

$ Sigma_"t,homogenised" = Sigma_"t,fuel" + Sigma_"t,graphite" $


= 3. 


#bibliography("main.bib",
style: "chicago-author-date")

