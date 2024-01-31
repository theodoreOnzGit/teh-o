
use uom::si::f64::*;
use uom::ConstZero;
use uom::si::length::centimeter;

use super::SurfaceTraits;
use crate::simulation::monte_carlo::openmc::position::{Position, Direction};
/// A general plane.
///
/// The plane is described by the equation A x + B y + C z - D = 0
#[derive(Debug,Clone,Copy,PartialEq, PartialOrd)]
pub struct SurfacePlane {
    #[allow(non_snake_case)]
    A_: ReciprocalLength,
    #[allow(non_snake_case)]
    B_: ReciprocalLength,
    #[allow(non_snake_case)]
    C_: ReciprocalLength,
    #[allow(non_snake_case)]
    D_: Ratio,
}


impl SurfacePlane {

    /// evaluates: 
    /// f(x,y,z) = A x + B y + C z - D 
    pub fn evaluate(&self, r: &Position) -> Ratio {
        return self.A_ * r.x + self.B_ * r.y + self.C_ * r.z - self.D_;
    }

    pub fn normal(&self) -> Direction {
        return Direction {
            x: Length::ZERO,
            y: Length::new::<centimeter>(1.0),
            z: Length::ZERO,
        };
    }
    // todo: bounding box
}
