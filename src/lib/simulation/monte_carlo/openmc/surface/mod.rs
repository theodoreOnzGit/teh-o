use std::collections::HashMap;

use super::position::{Position, Direction};
use uom::si::f64::*;

pub struct SurfaceModel {
    surface_map: HashMap<i64,i64>,
    // TBD: vector<unique_ptr<Surface>> surfaces;
}


// probably want to enum or trait over here
// cannot direct copy C++ polymorphism
pub struct Surface {

}

impl Default for Surface {
    fn default() -> Self {
        todo!()
    }
}
// this is FP_COINCIDENT constexpr double FP_COINCIDENT {1e-12};
const FP_COINCIDENT: f64 = 1e-12;

pub trait SurfaceTraits {
    // Determine which side of a surface a point lies on.
    // \param r The 3D Cartesian coordinate of a point.
    // \param u A direction used to "break ties" and pick a sense when the
    //   point is very close to the surface.
    // \return true if the point is on the "positive" side of the surface and
    //   false otherwise.
    fn sense(r: Position, u: Direction) -> bool {

        // OpenMC code:
        // // Evaluate the surface equation at the particle's coordinates to determine
        // // which side the particle is on.
        // const double f = evaluate(r);
        let f: f64 = Self::evaluate(r);

        // // Check which side of surface the point is on.
        // if (std::abs(f) < FP_COINCIDENT) {
        //     // Particle may be coincident with this surface. To determine the sense, we
        //     // look at the direction of the particle relative to the surface normal (by
        //     // default in the positive direction) via their dot product.
        //     return u.dot(normal(r)) > 0.0;
        // }
        if f.abs() < FP_COINCIDENT {
            return u.dot(&Self::normal(r)).value > 0.0;
        }
        return f > 0.0;

    }

    // Evaluate the equation describing the surface.
    //
    // Surfaces can be described by some function f(x, y, z) = 0.  This member
    // function evaluates that mathematical function.
    // \param r A 3D Cartesian coordinate.
    fn evaluate(r: Position) -> f64;
    // Compute the local outward normal direction of the surface.
    // \param r A 3D Cartesian coordinate.
    // \return Normal direction
    fn normal(r: Position) -> Direction;

    // Compute the distance between a point and the surface along a ray.
    // \param r A 3D Cartesian coordinate.
    // \param u The direction of the ray.
    // \param coincident A hint to the code that the given point should lie
    //   exactly on the surface.
    fn distance(r: Position, u: Direction, coincident: bool) -> Length;
}

/// X, Y and Z Planes 
pub mod xyz_planes;

/// Generic Plane 
pub mod generic_plane;

/// X, Y and Z Cylinders 
pub mod xyz_cylinders;

/// Spheres 
pub mod spheres;
