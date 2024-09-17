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

        pub fn scalar_mul(&self, point: Point, n: &mut i64) -> Point {
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
        pub fn gen_key_pair(&self, ord: i64) -> Result<(i64, Point), String> {
            if ord <= 1 {
                return Err("the value of n must be greater than 1.".to_string());
            }

            // generate a random number between 1 and ord - 1
            let mut rng = rand::thread_rng();
            let private_key = rng.gen_range(1..ord);

            // generate public key using scalar multiplication (private key: random integer)
            let public_key = self.scalar_mul(self.get_base_point(), &mut private_key.clone());

            Ok((private_key, public_key))
        }

        // function that returns a base point G of the curve (group generator)
        fn get_base_point(&self) -> Point {
            // conclude
            Point::new(10, 16)
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
        
            // Calcula el ancho máximo para cada coordenada (x, y)
            let max_width = points.iter()
                .flat_map(|p| [p.x.to_string(), p.y.to_string()])
                .map(|s| s.len())
                .max()
                .unwrap_or(0) + 2;
        
            // Imprime la primera fila (símbolo de infinito)
            println!("+{:-<width$}", "", width = (num_points + 1) * (max_width + 3) - 1);
        
            // Imprime la segunda fila (símbolo de suma y puntos)
            print!("| ∞");
            for point in points {
                print!("| {:width$} ", format!("({},{})", point.x, point.y), width = max_width);
            }
            println!("|");
        
            // Imprime las filas de la tabla
            for i in 0..num_points {
                print!("| {:width$} ", format!("({},{})", points[i].x, points[i].y), width = max_width);
                for j in 0..num_points {
                    let result = self.point_add(&points[i], &points[j]);
                    print!("| {:width$} ", format!("({},{})", result.x, result.y), width = max_width);
                }
                println!("|");
            }
        
            // Imprime la fila inferior
            println!("+{:-<width$}", "", width = (num_points + 1) * (max_width + 3) - 1);
        }

        /* pub fn print_cayley_table(&self, table: &[Vec<Point>]) {
            let num_points = table.len();
            let header_width = 10; // Adjust as needed

            // Print header row
            print!("{: <width$}", "+", width = header_width);
            for point in table[0].iter() {
                print!("{: <width$}", point, width = header_width);
            }
            println!();

            // Print remaining rows
            for i in 0..num_points {
                print!("{: <width$}", table[i][0], width = header_width);
                for j in 1..num_points {
                    print!("{: <width$}", table[i][j], width = header_width);
                }
                println!();
            }
        } */
    }
}
