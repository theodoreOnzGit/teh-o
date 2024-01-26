use uom::si::f64::*;
pub mod math_ops;
pub mod vector_ops;


#[derive(Debug,PartialEq, PartialOrd, Copy, Clone)]
pub struct Position {
    x: Length,
    y: Length,
    z: Length,
}



