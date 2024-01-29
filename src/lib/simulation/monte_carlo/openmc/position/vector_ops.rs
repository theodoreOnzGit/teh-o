use uom::si::{f64::*, length::meter};

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
    pub fn cross(&self, rhs: &Self) -> [Length;3] {

        let x = self.y * rhs.z.get::<meter>() - self.z * rhs.y.get::<meter>();
        let y = self.z * rhs.x.get::<meter>() - self.x * rhs.z.get::<meter>();
        let z = self.x * rhs.y.get::<meter>() - self.y * rhs.x.get::<meter>();

        return [x,y,z];

    }


}
