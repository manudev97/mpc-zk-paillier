pub mod basic_op {
        pub fn gcd(a: i64 , b: i64) -> i64 {
            if b == 0 { a } else { gcd(b, a % b)}
        }
        
        pub fn lcm(a: i64, b: i64) -> i64 {
            (a * b) / gcd(a,b)
        }

        pub fn inv_mod(x: i64, p: i64) -> Option<i64> {
            // if x is negative, adjust it by adding p
            let mut x = x;
            if x < 0 { x += p; }
        
            // finding the multiplicative inverse using exhaustive search
            for i in 1..p {
                if (x * i) % p == 1 {
                    return Some(i); // if found, return it
                }
            }

            None // if no inverse is found, return None
        }

        pub fn div_mod(x: i64, p: i64) -> Option<i64> {
            match inv_mod(x, p) {
                Some(inv) => Some((x * inv) % p),
                None => None,
            }
        }

        pub fn totient(n: &usize) -> usize {
            let mut result = *n;
            let mut n1 = *n;
            let mut p = 2;
        
            // Probar todos los factores primos de n
            while p * p <= *n {
                // Si p es un factor primo de n
                if n1 % p == 0 {
                    // Eliminar todos los factores p de n
                    while n1 % p == 0 {
                        n1 /= p;
                    }
                    // Aplicar la fórmula de Euler: φ(n) = n * (1 - 1/p)
                    result -= result / p;
                }
                p += 1;
            }
        
            // Si n es un número primo mayor que √n
            if n1 > 1 {
                result -= result / n1;
            }
        
            result
        }

    }