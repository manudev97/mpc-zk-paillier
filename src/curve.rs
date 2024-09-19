pub mod ecc {

    use crate::arithmetic::basic_op;
    extern crate rand;
    use rand::Rng;

    // definition of a Point structure to represent points on the curve
    #[derive(Debug, Copy, Clone)]
    pub struct Point {
        pub x: i64,
        pub y: i64,
    }

    impl Point {
        pub fn new(new_x: i64, new_y: i64) -> Self {
            Self { x: new_x, y: new_y }
        }
        
        pub fn to_string(&self) -> String {
            format!("({},{})",self.x, self.y)
        }
    }

    pub struct KeyPair {
        pub sk: usize,
        pub pk: Point,
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
            (point.y * point.y) % self.p
                == (point.x * point.x * point.x + self.a * point.x + self.b) % self.p
        }

        pub fn point_add(&self, point_a: &Point, point_b: &Point) -> Point {
            if point_a.x == 0 && point_a.y == 0 {
                return *point_b;
            } else if point_b.x == 0 && point_b.y == 0 {
                return *point_a;
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

                if x3 < 0 {
                    x3 += self.p;
                }
                if y3 < 0 {
                    y3 += self.p;
                }

                Point::new(x3, y3)
            }
        }

        pub fn scalar_mul(&self, point: Point, n: &mut usize) -> Point {
            let mut point_q = point;
            let mut point_r = Point::new(0, 0);
            while *n > 0 {
                if *n % 2 == 1 {
                    point_r = EcWei::point_add(&self, &point_r, &point_q);
                }
                point_q = EcWei::point_add(&self, &point_q, &point_q);
                *n /= 2;
                if *n == 0 {
                    break;
                }
            }
            point_r
        }

        /* // diffie-hellman
        pub fn dh(&self, point: &Point, n: &mut i64) -> Point {
            let point_q = EcWei::scalar_mul(&self, *point, n);
            point_q
        }*/

        // generate a key pair: private and public
        pub fn gen_key_pair(&self, group_points: Vec<Point>) -> Result<KeyPair, String> {
            let ord = group_points.len();
            if ord <= 1 {
                return Err("the value of n must be greater than 1.".to_string());
            }

            // generate a random number between 1 and ord - 1
            let mut rng = rand::thread_rng();
            let private_key = rng.gen_range(1..ord);

            // generate public key using scalar multiplication (private key: random integer)
            let public_key = self.scalar_mul(*self.get_base_points(&group_points).get(0).unwrap(), &mut private_key.clone());

            Ok(KeyPair{
                sk: private_key,
                pk: public_key,
            })
        }

        // function that returns a base point G of the curve (group generator)
        pub fn get_base_points<'a>(&self, group_points: &'a Vec<Point>) -> Vec<Point> {
            let mut n: usize = group_points.len()+1;
            let totient_n = basic_op::totient(&mut n);
            let mut generator_points = Vec::new();
        
            // we go through each point in the group
            for point in group_points {
                // we go through the divisors of n
                for k in 1..=totient_n {
                    if n % k as usize == 0 {
                        // we multiply the point by the divisor k
                        let mut k_usize = k as usize;
                        let result = self.scalar_mul(*point, &mut k_usize);
        
                        // if the result is the point at infinity (0,0), the point is a generator
                        if result.x == 0 && result.y == 0 {
                            generator_points.push(*point);
                            break;
                        }
                    }
                }
            }
        
            generator_points
        }

        pub fn group_points(&self) -> Vec<Point> {
            let mut points = Vec::new();
            for x in 0..self.p {
                for y in 0..self.p {
                    let point = Point::new(x, y);
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

            // calculate the size of the cells (max_x_len and max_y_len)
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

            // function to print a top or bottom line of the table
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
                    if result.x == 0 && result.y == 0 {
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
