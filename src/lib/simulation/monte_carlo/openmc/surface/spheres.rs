use uom::si::f64::*;
use uom::ConstZero;
use uom::si::length::centimeter;

use super::SurfaceTraits;
use crate::simulation::monte_carlo::openmc::position::{Position, Direction};

#[derive(Debug,Clone,Copy,PartialEq, PartialOrd)]
pub struct SurfaceSphere {
    x0_: Length,
    y0_: Length,
    z0_: Length,
    radius_: Length,
}

impl SurfaceSphere {
    pub fn evaluate(&self, r: &Position) -> Area {

        let x = r.x - self.x0_;
        let y = r.y - self.y0_;
        let z = r.z - self.z0_;

        return x * x + y * y + z * z - self.radius_ * self.radius_;
    }

    pub fn normal(&self, r: &Position) -> Direction {
        //
        // direct translate of:
        //return {2.0 * (r.x - x0_), 2.0 * (r.y - y0_), 2.0 * (r.z - z0_)};

        let x = r.x - self.x0_;
        let y = r.y - self.y0_;
        let z = r.z - self.z0_;

        return Direction {
            x: 2.0 * x,
            y: 2.0 * y,
            z: 2.0 * z,
        };
    }

    // todo: bounding box
}
