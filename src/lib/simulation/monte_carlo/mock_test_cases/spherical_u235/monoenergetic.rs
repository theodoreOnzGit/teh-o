// in this demonstration case,
// we consider a sphere of u235 with monoenergetic cross sections to 
// simplify the problem 
// 
//
// A neutron would traverse the system,
// contact u235 metal (100% enriched 
// for simplicity) 
//
// noting that the critical mass is about 50 kg for u235, we can make 
// a sphere of uranium with vacuum BCs
// 
// The main things to get right are the:
//
// 1. random walk process 
// 2. sampling of new fission sites 
// 3. geometry and handling leakage
// 
// For this, we can consider one energy group (fast) and base our cross
// sections upon the fast neutron spectrum. of about 1 MeV



