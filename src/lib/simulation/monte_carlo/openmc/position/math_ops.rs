use std::ops::{Add, Sub, AddAssign, SubAssign};
use super::Position;

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

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        let x2 = rhs.x;
        let y2 = rhs.y;
        let z2 = rhs.z;

        let x1 = self.x;
        let y1 = self.y;
        let z1 = self.z;

        self.x = x1 + x2;
        self.y = y1 + y2;
        self.z = z1 + z2;
    }
}

impl SubAssign for Position {
    fn sub_assign(&mut self, rhs: Self) {
        let x2 = rhs.x;
        let y2 = rhs.y;
        let z2 = rhs.z;

        let x1 = self.x;
        let y1 = self.y;
        let z1 = self.z;

        self.x = x1 - x2;
        self.y = y1 - y2;
        self.z = z1 - z2;
    }
}
#[test]
pub fn test_position_add_subtract_ops(){

    use uom::si::f64::*;
    use uom::si::length::meter;

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
    assert_abs_diff_eq!(pos_3.y.get::<meter>(),
        0.3, 
        epsilon = f64::EPSILON,
    );
    assert_abs_diff_eq!(pos_3.z.get::<meter>(),
        0.3, 
        epsilon = f64::EPSILON,
    );

    let pos_3 = pos_1 - pos_2;

    assert_abs_diff_eq!(pos_3.x.get::<meter>(),
        -0.1, 
        epsilon = f64::EPSILON,
    );
    assert_abs_diff_eq!(pos_3.y.get::<meter>(),
        -0.1, 
        epsilon = f64::EPSILON,
    );
    assert_abs_diff_eq!(pos_3.z.get::<meter>(),
        -0.1, 
        epsilon = f64::EPSILON,
    );


}

#[test]
pub fn test_position_add_sub_assign_ops(){
    use uom::si::f64::*;
    use uom::si::length::meter;

    let pos_1: Position = Position { 
        x: Length::new::<meter>(0.1), 
        y: Length::new::<meter>(0.1), 
        z: Length::new::<meter>(0.1),
    };

    let mut pos_4: Position = Position { 
        x: Length::new::<meter>(0.4), 
        y: Length::new::<meter>(0.4), 
        z: Length::new::<meter>(0.4),
    };

    pos_4 += pos_1;

    
    assert_abs_diff_eq!(pos_4.x.get::<meter>(),
        0.5, 
        epsilon = f64::EPSILON,
    );
    assert_abs_diff_eq!(pos_4.y.get::<meter>(),
        0.5, 
        epsilon = f64::EPSILON,
    );
    assert_abs_diff_eq!(pos_4.z.get::<meter>(),
        0.5, 
        epsilon = f64::EPSILON,
    );

    pos_4 -= pos_1;
    pos_4 -= pos_1;

    assert_abs_diff_eq!(pos_4.x.get::<meter>(),
        0.3, 
        epsilon = f64::EPSILON,
    );
    assert_abs_diff_eq!(pos_4.y.get::<meter>(),
        0.3, 
        epsilon = f64::EPSILON,
    );
    assert_abs_diff_eq!(pos_4.z.get::<meter>(),
        0.3, 
        epsilon = f64::EPSILON,
    );
}
