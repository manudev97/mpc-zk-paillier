pub mod ecc {
    use crate::arithmetic::basic_op;
    extern crate rand;
    use num_bigint::BigInt;
    use num_traits::{One, ToPrimitive, Zero};
    use rand::Rng;

    // definition of the Point structure to represent points on the curve
    #[derive(Debug, Clone, PartialEq)]
    pub struct Point {
        pub x: BigInt,
        pub y: BigInt,
    }

    impl Point {
        pub fn new(new_x: BigInt, new_y: BigInt) -> Self {
            Self { x: new_x, y: new_y }
        }

        pub fn to_string(&self) -> String {
            format!("({}, {})", self.x, self.y)
        }
    }

    #[derive(Debug, Clone)]
    pub struct KeyPair {
        pub sk: BigInt,
        pub pk: Point,
    }

    // definition of the EcWei structure to represent the elliptic curve
    #[derive(Debug)]
    pub struct EcWei {
        a: BigInt,
        b: BigInt,
        p: BigInt,
    }

    impl EcWei {
        pub fn new(new_a: BigInt, new_b: BigInt, new_p: BigInt) -> Self {
            Self {
                a: new_a,
                b: new_b,
                p: new_p,
            }
        }

        pub fn is_point(&self, point: &Point) -> bool {
            let lhs = (&point.y * &point.y) % &self.p;
            let rhs = (&point.x * &point.x * &point.x + &self.a * &point.x + &self.b) % &self.p;
            lhs == rhs
        }

        pub fn point_add(&self, point_a: &Point, point_b: &Point) -> Point {
            if point_a.x.is_zero() && point_a.y.is_zero() {
                return point_b.clone();
            } else if point_b.x.is_zero() && point_b.y.is_zero() {
                return point_a.clone();
            } else if point_a.x == point_b.x && point_a.y == -&point_b.y {
                return Point::new(BigInt::zero(), BigInt::zero());
            } else if point_a.x == point_b.x && point_a.y != point_b.y {
                return Point::new(BigInt::zero(), BigInt::zero());
            } else {
                let l;
                // P != Q
                if point_a.x != point_b.x || point_a.y != point_b.y {
                    let numerator = &point_b.y - &point_a.y;
                    let denominator = &point_b.x - &point_a.x;
                    l = basic_op::inv_mod(&denominator.clone(), &self.p.clone()).unwrap()
                        * numerator
                        % &self.p;
                } else {
                    // P == Q
                    let numerator = BigInt::from(3) * &point_a.x * &point_a.x + &self.a;
                    let denominator = BigInt::from(2) * &point_a.y;
                    l = basic_op::inv_mod(&denominator.clone(), &self.p.clone()).unwrap()
                        * numerator
                        % &self.p;
                }

                let mut x3 = (&l * &l - &point_a.x - &point_b.x) % &self.p;
                let mut y3 = (&l * (&point_a.x - x3.clone()) - &point_a.y) % &self.p;

                if x3 < BigInt::zero() {
                    x3 += &self.p;
                }
                if y3 < BigInt::zero() {
                    y3 += &self.p;
                }

                Point::new(x3, y3)
            }
        }

        pub fn scalar_mul(&self, point: &Point, d: &BigInt) -> Point {
            let mut n = d.clone();
            let mut point_q = point.clone();
            let mut point_r = Point::new(BigInt::zero(), BigInt::zero());

            while n > BigInt::zero() {
                if &n % BigInt::from(2) == BigInt::one() {
                    point_r = self.point_add(&point_r, &point_q);
                }
                point_q = self.point_add(&point_q, &point_q);
                n /= 2;
            }

            point_r
        }

        // generate a key pair: private and public
        pub fn gen_key_pair(&self, generator: &Point) -> Result<KeyPair, String> {
            let ord = BigInt::from(self.group_points().len());
            if ord <= BigInt::one() {
                return Err("the value of n must be greater than 1.".to_string());
            }

            // generate a random number between 2 and ord - 1
            let mut rng = rand::thread_rng();
            let random_u64: u64 = rng.gen_range(2..ord.to_u64().unwrap());
            let private_key = BigInt::from(random_u64);

            // generate the public key using scalar multiplication
            let public_key = self.scalar_mul(generator, &private_key.clone());

            Ok(KeyPair {
                sk: private_key,
                pk: public_key,
            })
        }

        pub fn get_base_points<'a>(&self, group_points: &'a Vec<Point>) -> Vec<Point> {
            let n = BigInt::from(group_points.len() + 1); // the order of the group
            let mut generator_points = Vec::new(); // vector to store the generating points

            for point in group_points {
                let mut is_generator = true;

                let mut k = BigInt::from(2);

                while k <= n {
                    if &n % &k == BigInt::zero() {
                        let result = self.scalar_mul(point, &k.clone());

                        // if k < n and the result is the identity point
                        if k < n && result.x.is_zero() && result.y.is_zero() {
                            is_generator = false;
                            break;
                        }
                    }

                    k += 1;
                }

                if is_generator {
                    generator_points.push(point.clone());
                }
            }

            generator_points
        }

        pub fn group_points(&self) -> Vec<Point> {
            let mut points = Vec::new();
            for x in 0..self.p.to_i64().unwrap() {
                for y in 0..self.p.to_i64().unwrap() {
                    let point = Point::new(BigInt::from(x), BigInt::from(y));
                    if self.is_point(&point) {
                        points.push(point);
                    }
                }
            }

            points
        }

        pub fn cayley_table(&self, points: &Vec<Point>) {
            let num_points = points.len();
            let ascii_title = r#"
 $$$$$$\   $$$$$$\ $$\     $$\ $$\       $$$$$$$$\ $$\     $$\ $$\  $$$$$$\        $$$$$$$$\  $$$$$$\  $$$$$$$\  $$\       $$$$$$$$\ 
$$  __$$\ $$  __$$\\$$\   $$  |$$ |      $$  _____|\$$\   $$  |$  |$$  __$$\       \__$$  __|$$  __$$\ $$  __$$\ $$ |      $$  _____|
$$ /  \__|$$ /  $$ |\$$\ $$  / $$ |      $$ |       \$$\ $$  / \_/ $$ /  \__|         $$ |   $$ /  $$ |$$ |  $$ |$$ |      $$ |      
$$ |      $$$$$$$$ | \$$$$  /  $$ |      $$$$$\      \$$$$  /      \$$$$$$\           $$ |   $$$$$$$$ |$$$$$$$\ |$$ |      $$$$$\    
$$ |      $$  __$$ |  \$$  /   $$ |      $$  __|      \$$  /        \____$$\          $$ |   $$  __$$ |$$  __$$\ $$ |      $$  __|   
$$ |  $$\ $$ |  $$ |   $$ |    $$ |      $$ |          $$ |        $$\   $$ |         $$ |   $$ |  $$ |$$ |  $$ |$$ |      $$ |      
\$$$$$$  |$$ |  $$ |   $$ |    $$$$$$$$\ $$$$$$$$\     $$ |        \$$$$$$  |         $$ |   $$ |  $$ |$$$$$$$  |$$$$$$$$\ $$$$$$$$\ 
 \______/ \__|  \__|   \__|    \________|\________|    \__|         \______/          \__|   \__|  \__|\_______/ \________|\________|
    "#;
            println!("{}", ascii_title);

            let max_x_len = points
                .iter()
                .map(|p| p.x.to_string().len())
                .max()
                .unwrap_or(0);
            let max_y_len = points
                .iter()
                .map(|p| p.y.to_string().len())
                .max()
                .unwrap_or(0);

            fn print_table_border(num_points: usize, cell_width: usize) {
                print!("+");
                for _ in 0..=num_points + 1 {
                    print!("{:-<width$}+", "", width = cell_width);
                }
                println!();
            }

            let cell_width = max_x_len + max_y_len + 5; // adjustment for cell space

            // print the top line of the table
            print_table_border(num_points, cell_width);

            // print the second row (plus symbol, infinity and dots)
            print!("|    +    |    ∞    ");
            for point in points {
                // format x and y coordinate with maximum size found
                print!(
                    "| ({:>width_x$},{:>width_y$}) ",
                    point.x,
                    point.y,
                    width_x = max_x_len,
                    width_y = max_y_len
                );
            }
            println!("|");

            print_table_border(num_points, cell_width);

            // print the third row (plus symbol and dots)
            print!("|    ∞    |    ∞    ");
            for point in points {
                // format x and y coordinate with maximum size found
                print!(
                    "| ({:>width_x$},{:>width_y$}) ",
                    point.x,
                    point.y,
                    width_x = max_x_len,
                    width_y = max_y_len
                );
            }
            println!("|");

            print_table_border(num_points, cell_width);

            // print the remaining rows of the table
            for point in points {
                // format x and y coordinate with maximum size found
                for _ in 0..2 {
                    print!(
                        "| ({:>width_x$},{:>width_y$}) ",
                        point.x,
                        point.y,
                        width_x = max_x_len,
                        width_y = max_y_len
                    );
                }
                for j in 0..num_points {
                    let result = self.point_add(&point, &points[j]);

                    // if the result is the point (0, 0), we print the infinity symbol
                    if result.x == BigInt::zero() && result.y == BigInt::zero() {
                        print!(
                            "| {:^width$} ",
                            "∞",
                            width = max_x_len + max_y_len + 3 // total space to align with the rest
                        );
                    } else {
                        // normally prints x and y coordinates
                        print!(
                            "| ({:>width_x$},{:>width_y$}) ",
                            result.x,
                            result.y,
                            width_x = max_x_len,
                            width_y = max_y_len
                        );
                    }
                }

                println!("|");
                print_table_border(num_points, cell_width);
            }
        }
    }
}
