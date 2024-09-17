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
    }