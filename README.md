# teh-o
Transport, Eigenvalue and Hybrid Open Source  Solver (teh-o) 

## Purpose 

A Free and Open Source Monte Carlo and/or deterministic solver library 
meant for neutron transport and coupled reactor multiphysics problems 
developed in Singapore.
It takes inspiration from OpenMC's algorithms and translates lots of its 
API to the Rust programming language. However, it is meant to make 
improvements and make the coupling with thermal hydraulics solvers 
easier. This could be a direct coupling of Monte Carlo with thermal 
hydraulics, or automatic MGXS generation for multiphysics deterministic 
code.

Some improvements I hope to include are unit safety, and cross platform 
capability.

## Pre-requisites

1. cmake

## Nomenclature 

It is named after a beloved local beverage, teh-o (or black tea in Hokkien).

# Credit 

Most of the monte carlo code comes from directly translating OpenMC

Paul K. Romano, Nicholas E. Horelik, Bryan R. Herman, Adam G. 
Nelson, Benoit Forget, and Kord Smith, "OpenMC: A 
State-of-the-Art Monte Carlo Code for Research and Development," 
Ann. Nucl. Energy, 82, 90--97 (2015).

OpenMC v0.14.0 was released under the MIT/X License 

Copyright © 2011-2023 Massachusetts Institute of Technology, 
UChicago Argonne LLC, and OpenMC contributors

Permission is hereby granted, free of charge, to any person 
obtaining a copy of this software and associated documentation 
files (the “Software”), to deal in the Software without restriction, 
including without limitation the rights to use, copy, modify, merge, 
publish, distribute, sublicense, and/or sell copies of the 
Software, and to permit persons to whom the Software is furnished to do 
so, subject to the following conditions:

The above copyright notice and this permission notice shall be 
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, 
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF 
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. 
IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY 
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, 
TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE 
SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

