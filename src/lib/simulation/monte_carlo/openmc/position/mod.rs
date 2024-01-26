use uom::si::f64::*;
pub mod math_ops;
pub mod vector_ops;


#[derive(Debug,PartialEq, PartialOrd, Copy, Clone)]
pub struct Position {
    x: Length,
    y: Length,
    z: Length,
}



#[test]
pub fn test_position_math_ops(){

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
