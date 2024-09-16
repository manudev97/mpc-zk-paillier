
pub mod ecc {
    use crate::arithmetic::basic_op;
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

        pub fn is_point(&self, point: &Point) -> bool {
            (point.y * point.y) % self.p == (point.x * point.x * point.x + self.a * point.x + self.b) % self.p
        }
    
        pub fn point_addition(&self, point_a: Point, point_b: Point) -> Point {
            if point_a.x == 0 && point_a.y == 0 {
                return point_b;
            } else if point_b.x == 0 && point_b.y == 0 {
                return point_a;
            } else if point_a.x == point_b.x && point_a.y == -point_b.y {
                return Point::new(0, 0);
            } else if point_a.x == point_b.x && point_a.y != point_b.y {
                return Point::new(0, 0);
            } else {
                let l;
                // P != Q
                if point_a.x != point_b.x || point_a.y != point_b.y {
                    let numerator = point_b.y - point_a.y;
                    let denominator = point_b.x - point_a.x;
                    l = basic_op::inv_mod(denominator, self.p).unwrap() * numerator % self.p;
                } else {
                    // P == Q
                    let numerator = 3 * point_a.x * point_a.x + self.a;
                    let denominator = 2 * point_a.y;
                    l = basic_op::inv_mod(denominator, self.p).unwrap() * numerator % self.p;
                }

                let mut x3 = (l * l - point_a.x - point_b.x) % self.p;
                let mut y3 = (l * (point_a.x - x3) - point_a.y) % self.p;

                if x3 < 0 { x3 += self.p; }
                if y3 < 0 { y3 += self.p; }

                Point::new(x3, y3)
            }
        }
    
    }

}
