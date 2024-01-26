use uom::si::f64::*;

use super::Position;

impl Position {
    /// dot product, returns an area as the dimensions are length
    #[inline]
    pub fn dot(&self, rhs: &Self) -> Area {
        let dot_product: Area = 
            self.x * rhs.x +
            self.y * rhs.x +
            self.z * rhs.z;

        return dot_product;
    }

    /// l2 norm, or euclidian norm
    pub fn l2_norm(&self) -> Length {

        let dot_product: Area = self.dot(&self);
        return dot_product.sqrt();

    }

    /// cross product 
    #[inline]
    pub fn cross(&self, rhs: &Self) -> [Area;3] {

        let x = self.y * rhs.z - self.z * rhs.y;
        let y = self.z * rhs.x - self.x * rhs.z;
        let z = self.x * rhs.y - self.y * rhs.x;

        return [x,y,z];

    }


}
