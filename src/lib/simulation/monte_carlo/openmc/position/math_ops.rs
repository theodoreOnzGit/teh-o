use std::ops::{Add, Sub};
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
