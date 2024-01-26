use std::ops::{Add, Sub};

use uom::si::f64::*;
use uom::si::length::meter;


#[derive(Debug,PartialEq, PartialOrd, Copy, Clone)]
pub struct Position {
    x: Length,
    y: Length,
    z: Length,
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let x2 = rhs.x;
        let y2 = rhs.y;
        let z2 = rhs.z;

        let x1 = self.x;
        let y1 = self.y;
        let z1 = self.z;

        let x = x1 + x2;
        let y = y1 + y2;
        let z = z1 + z2;

        return Self {
            x, y, z
        };

    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let x2 = rhs.x;
        let y2 = rhs.y;
        let z2 = rhs.z;

        let x1 = self.x;
        let y1 = self.y;
        let z1 = self.z;

        let x = x1 - x2;
        let y = y1 - y2;
        let z = z1 - z2;

        return Self {
            x, y, z
        };

    }
}


#[test]
pub fn test_position_math_ops(){


    let pos_1: Position = Position { 
        x: Length::new::<meter>(0.1), 
        y: Length::new::<meter>(0.1), 
        z: Length::new::<meter>(0.1),
    };
    let pos_2: Position = Position { 
        x: Length::new::<meter>(0.2), 
        y: Length::new::<meter>(0.2), 
        z: Length::new::<meter>(0.2),
    };

    let pos_3 = pos_1 + pos_2;

    assert_abs_diff_eq!(pos_3.x.get::<meter>(),
        0.3, 
        epsilon = f64::EPSILON,
    );
    let _pos_3 = pos_1 - pos_2;

}
