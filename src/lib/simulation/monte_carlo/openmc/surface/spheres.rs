use uom::si::area::{square_centimeter, square_meter};
use uom::si::f64::*;
use uom::si::ratio::ratio;
use uom::ConstZero;
use uom::si::length::{centimeter, meter};

use super::{SurfaceTraits, FP_COINCIDENT};
use crate::{simulation::monte_carlo::openmc::position::{Position, Direction}, teh_o_error::TehOError};

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
    //

    pub fn axis_aligned_cylinder_distance(
        &self,
        r: &Position,
        u: &Direction,
        coincident: bool,
        radius: Length,) -> Result<Length, TehOError>{

        //const double x = r.x - x0_;
        //const double y = r.y - y0_;
        //const double z = r.z - z0_;
        //const double k = x * u.x + y * u.y + z * u.z;
        //const double c = x * x + y * y + z * z - radius_ * radius_;
        //const double quad = k * k - c;

        let x: Length = r.x - self.x0_;
        let y: Length = r.y - self.y0_;
        let z: Length = r.z - self.z0_;
        // let's re-define the u so that u is a proper unit vector 
        // This time, we use SI units (meters)


        let magnitude_of_u: Length = u.dot(&u).sqrt();
        let u_x_dimensionless: Ratio = u.x / magnitude_of_u;
        let u_y_dimensionless: Ratio = u.y / magnitude_of_u;
        let u_z_dimensionless: Ratio = u.z / magnitude_of_u;

        let u_dimensionless: [Ratio;3] = 
            [u_x_dimensionless, u_y_dimensionless, u_z_dimensionless];

        let k: Length = x * u_x_dimensionless + y * u_y_dimensionless + 
            z * u_z_dimensionless;

        let c: Area = x * x + y * y + z * z - self.radius_ * self.radius_;
        let quad: Area = k * k - c;

        let infinite_length = Length::new::<meter>(f64::INFINITY);

        
        // Translate direct:
        // if (quad < 0.0) {
        //   // No intersection with sphere.
        //   return INFTY;

        // } else if (coincident || std::abs(c) < FP_COINCIDENT) {
        //   // Particle is on the sphere, thus one distance is positive/negative and
        //   // the other is zero. The sign of k determines if we are facing in or out.
        //   if (k >= 0.0) {
        //     return INFTY;
        //   } else {
        //     return -k + sqrt(quad);
        //   }

        // } else if (c < 0.0) {
        //   // Particle is inside the sphere, thus one distance must be negative and
        //   // one must be positive. The positive distance will be the one with
        //   // negative sign on sqrt(quad)
        //   return -k + sqrt(quad);

        // } else {
        //   // Particle is outside the sphere, thus both distances are either positive
        //   // or negative. If positive, the smaller distance is the one with positive
        //   // sign on sqrt(quad).
        //   const double d = -k - sqrt(quad);
        //   if (d < 0.0)
        //     return INFTY;
        //   return d;
        // }

        if quad < Area::ZERO {
            // No intersection with sphere.
            return Ok(infinite_length);
        } else if (coincident || c.get::<square_meter>() < FP_COINCIDENT ) {
            // Particle is on the sphere, thus one distance is positive/negative and
            // the other is zero. The sign of k determines if we are facing in or out.
            if (k.value >= 0.0) {
                return Ok(infinite_length);
            } else {
                return Ok(-k + quad.sqrt()) ;

            }
        } else if c < Area::ZERO {
            // Particle is inside the sphere, thus one distance must be negative
            // and one must be positive. The positive distance will be the one with
            // negative sign on sqrt(quad).

            return Ok(-k + quad.sqrt() );
        } else {
            // Particle is outside the sphere, thus both distances are either
            // positive or negative. If positive, the smaller distance is the one
            // with positive sign on sqrt(quad).
            let d = -k - quad.sqrt();
            if d < Length::ZERO {
                return Ok(infinite_length);
            }
            return Ok(d);
        }

    }
}
