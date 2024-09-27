use num_bigint::BigInt;
use num_traits::{One, Zero};

pub mod basic_op {
    use super::*;

    pub fn gcd(a: &BigInt, b: &BigInt) -> BigInt {
        if b.is_zero() {
            a.clone()
        } else {
            gcd(b, &(a % b))
        }
    }
    
    pub fn lcm(a: &BigInt, b: &BigInt) -> BigInt {
        (a * b) / gcd(a, b)
    }

    pub fn inv_mod(x: &BigInt, p: &BigInt) -> Option<BigInt> {
        // if x is negative, adjust it by adding p
        let mut x = x.clone();
        if x < BigInt::zero() {
            x += p;
        }

        // finding the multiplicative inverse using exhaustive search
        let mut i = BigInt::one();
        while &i < p {
            if (x.clone() * &i) % p == BigInt::one() {
                return Some(i.clone()); // if found, return it
            }
            i += BigInt::one(); // Increment i
        }

        None // if no inverse is found, return None
    }

    pub fn div_mod(x: &BigInt, p: &BigInt) -> Option<BigInt> {
        match inv_mod(x, p) {
            Some(inv) => Some((x.clone() * inv) % p),
            None => None,
        }
    }

}
