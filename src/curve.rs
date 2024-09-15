pub mod ecc {
    // definition of a Point structure to represent points on the curve
    #[derive(Debug)]
    pub struct Point {
        pub x: i64,
        pub y: i64,
    }

    impl Point {
        pub fn new(new_x: i64, new_y: i64) -> Self {
            Self {
            x: new_x,
            y: new_y,
            }
        }
    }

    // definition of an EcWei structure to represent the elliptic curve
    #[derive(Debug)]
    pub struct EcWei {
        a: i64,
        b: i64,
        p: i64,
    }

    impl EcWei {
        pub fn new(new_a: i64, new_b: i64, new_p: i64) -> Self {
            Self {
                a: new_a,
                b: new_b,
                p: new_p,
            }
        }

        pub fn is_point(self, point: &Point) -> bool {
            (point.y * point.y) % self.p == (point.x * point.x * point.x + self.a * point.x + self.b) % self.p
        }
    }
    
    mod basic_op {
        
    } 

}
