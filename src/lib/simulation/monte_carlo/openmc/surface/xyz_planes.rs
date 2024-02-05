use uom::si::f64::*;
use uom::ConstZero;
use uom::si::length::centimeter;

use crate::simulation::monte_carlo::openmc::position::{Position, Direction};

#[derive(Debug,Clone,Copy,PartialEq, PartialOrd)]
pub struct SurfaceXPlane {
    x0_: Length,
}

impl SurfaceXPlane {
    pub fn evaluate(&self, r: &Position) -> Length {
        return r.x - self.x0_;
    }

    pub fn normal(&self) -> Direction {
        return Direction {
            x: Length::new::<centimeter>(1.0),
            y: Length::ZERO,
            z: Length::ZERO,
        };
    }

    // todo: bounding box
}

#[derive(Debug,Clone,Copy,PartialEq, PartialOrd)]
pub struct SurfaceYPlane {
    y0_: Length,
}

impl SurfaceYPlane {

    pub fn evaluate(&self, r: &Position) -> Length {
        return r.y - self.y0_;
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

#[derive(Debug,Clone,Copy,PartialEq, PartialOrd)]
pub struct SurfaceZPlane {
    z0_: Length,
}

impl SurfaceZPlane {

    pub fn evaluate(&self, r: &Position) -> Length {
        return r.z - self.z0_;
    }

    pub fn normal(&self) -> Direction {
        return Direction {
            x: Length::ZERO,
            y: Length::ZERO,
            z: Length::new::<centimeter>(1.0),
        };
    }
    // todo: bounding box
}
