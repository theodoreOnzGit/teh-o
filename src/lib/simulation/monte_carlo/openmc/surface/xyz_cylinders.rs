use std::ops::Deref;

use uom::si::area::{square_centimeter, square_meter};
use uom::si::f64::*;
use uom::si::ratio::ratio;
use uom::ConstZero;
use uom::si::length::{centimeter, meter};

use super::SurfaceTraits;
use crate::simulation::monte_carlo::openmc::surface::FP_COINCIDENT;
use crate::teh_o_error::TehOError;
use crate::simulation::monte_carlo::openmc::position::{Position, Direction};

#[inline]
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

// The first template parameter (i1) indicates which axis the cylinder is aligned to.
// The other two parameters, (i2 and i3) 
// indicate the other two axes.  offset1 and offset2
// should correspond with i2 and i3, respectively.
//
// how is this supposed to work exactly?
#[inline]
pub fn axis_aligned_cylinder_distance(
    r: &Position,
    i1: u8,
    i2: u8,
    i3: u8,
    u: &Direction,
    coincident: bool,
    offset1: Length,
    offset2: Length,
    radius: Length,) -> Result<Length, TehOError>{

    // how do we measure distance from a point to a point lying on a cylidner?
    //
    // in 3D space, we need to define a point on the cylinder surface before 
    // we can calculate a distance. 
    //
    // But which point? 
    //
    // For this, we need to consider a few cases. 
    // First, if the particle is inside the cylinder, then we use 
    // a set algorithm 
    //
    // if it is outside the cylinder, then we use another algorithm
    //
    // if it's inside the cylinder, we can specify a unit vector u 
    // (I think openmc uses centimeters) that we can extend all the 
    // way until there is intersection with the cylinder
    //
    // The only exception is that if u is parallel to the cylinder. Then 
    // we meet the cylinder at infinity 
    // 
    // Let us handle the fringe case first assuming that we have u pointing 
    // to infinity assuming u is a unit vector in cm
    //
    // This is the openmc implementation
    let a = Area::new::<square_centimeter>(1.0) - u.get(i1)?* u.get(i1)?;
    let infinite_length = Length::new::<meter>(f64::INFINITY);
    if a == Area::ZERO {
        return Ok(infinite_length);
    }

    // if we specify other units, then we are in trouble, how shall we 
    // handle this?
    //
    // let's re-define the u so that u is a proper unit vector 
    // This time, we use SI units (meters)
    

    let magnitude_of_u: Length = u.dot(&u).sqrt();
    let u_x_dimensionless: Ratio = u.x / magnitude_of_u;
    let u_y_dimensionless: Ratio = u.y / magnitude_of_u;
    let u_z_dimensionless: Ratio = u.z / magnitude_of_u;

    let u_dimensionless: [Ratio;3] = 
        [u_x_dimensionless, u_y_dimensionless, u_z_dimensionless];

    // need to do some type conversions
    let i1_usize: usize = i1.try_into().unwrap();
    let i2_usize: usize = i2.try_into().unwrap();
    let i3_usize: usize = i3.try_into().unwrap();

    let u_i1 = u_dimensionless.get(i1_usize)
        .ok_or(TehOError::OpenMcErrOutOfBounds)?
        .to_owned();
    let u_i2 = u_dimensionless.get(i2_usize)
        .ok_or(TehOError::OpenMcErrOutOfBounds)?
        .to_owned();
    let u_i3 = u_dimensionless.get(i3_usize)
        .ok_or(TehOError::OpenMcErrOutOfBounds)?
        .to_owned();

    let a = Ratio::new::<ratio>(1.0) - u_i1 * u_i1;
    if a == Ratio::ZERO {
        return Ok(infinite_length);
    }


    // now, this will handle other units, or in case u is not a unit vector
    // slightly more expensive, but ok.

    // now with the fringe case handled, we can move on. 
    //
    // if we don't have infinite distance, 
    // Then we measure the distance from the point to the circle
    //
    // in 2D, it looks like
    //
    //      ---------
    //    /          \
    //   /            \
    //  /              \
    // |               |
    // |       c  pt-->x
    // |               |
    //  \             /
    //   \           /
    //    \         /
    //      ------- 
    //
    // How can we find x? 
    //
    // Well, using vectors, we can help ourselves find x at least for 
    // a 2D case.
    //
    // I suppose there is some long derivation I need to do, or at least look 
    // online for. But I'll just translate the OpenMC code
    //
    let r2: Length = r.get(i2)? - offset1;
    let r3: Length = r.get(i3)? - offset2;
    // k is some projection calculation, therefore, using a dimensionless 
    // unit vector is ok
    let k: Length = r2 * u_i2 + r3 * u_i3;
    let c: Area = r2 * r2 + r3 * r3 - radius * radius;
    let quad: Area = k * k - a * c; 

    if quad < Area::ZERO {
        // No intersection with cylinder.
        return Ok(infinite_length);
    } else if (coincident || c.get::<square_meter>() < FP_COINCIDENT ) {
        // Particle is on the cylinder, thus one distance is positive/negative
        // and the other is zero. The sign of k determines if we are facing in or
        // out.
        //
        if (k.value >= 0.0) {
            return Ok(infinite_length);
        } else {
            // todo return (-k + sqrt(quad)) / a;
            return Ok((-k + quad.sqrt()) / a);

        }
    } else if c < Area::ZERO {
        // Particle is inside the cylinder, thus one distance must be negative
        // and one must be positive. The positive distance will be the one with
        // negative sign on sqrt(quad).

        return Ok((-k + quad.sqrt()) / a);
    }





    todo!()

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

