use uom::si::f64::*;
use uom::ConstZero;
use uom::si::length::centimeter;

use super::SurfaceTraits;
use crate::{simulation::monte_carlo::openmc::position::{Position, Direction}, teh_o_error::TehOError};

pub fn axis_aligned_cylinder_evaluate(
    r: &Position,
    i1: u8,
    i2: u8,
    offset1: Length,
    offset2: Length,
    radius: Length,) -> Result<Area, TehOError>{

    let r1 = r.get(i1)? - offset1;
    let r2 = r.get(i2)? - offset2;
    return Ok(r1 * r1 + r2 * r2 - radius * radius);

}

///==============================================================================
/// A cylinder aligned along the x-axis.
///
/// The cylinder is described by the equation
/// (y - y_0)^2 + (z - z_0)^2 - R^2 = 0
///==============================================================================
#[derive(Debug,Clone,Copy,PartialEq, PartialOrd)]
pub struct SurfaceXCylinder {
    y0_: Length,
    z0_: Length,
    radius_: Length,
}

impl SurfaceXCylinder {
    pub fn evaluate(&self, r: &Position) -> Result<Area,TehOError> {
        return axis_aligned_cylinder_evaluate(
            r,
            1, 2,
            self.y0_, self.z0_, self.radius_);
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

