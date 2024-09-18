use crate::arithmetic::basic_op;
extern crate rand;
use rand::Rng;
use std::ops::Rem;

pub struct PaillierKey {
    pub public_key: (u128, u128),
    pub private_key: (u128, u128),
}

pub fn gen_key_paillier(p: u128, q: u128) -> PaillierKey {
    // calculate N
    let n = p * q;
    // calculate λ
    let lambda = lcm(p - 1, q - 1);

    // generate g randomly
    let mut rng = rand::thread_rng();
    let mut g = n;
    let mut l: u128 = n;
    
    // select g and calculate L until gcd(L, N) == 1
    while gcd(l, n) != 1 {
        g = rng.gen_range(1..n.pow(2));
        l = ((g.pow(lambda as u32) % n.pow(2)) - 1) / n;
    }

    // calculate μ using the modular inverse
    let mu = inv_mod(l, n);

    PaillierKey {
        public_key: (g, n),
        private_key: (lambda, mu),
    }
}

pub fn cipher_paillier(public_key: (u128, u128), m: u128) -> Result<u128, &'static str> {
    let (g, n) = public_key;

    // ensure that the M message is appropriate
    if m >= n || m <= 0 {
        return Err("El mensaje o secreto no es apropiado");
    }

    // generate r randomly, making sure that gcd(r, N) == 1
    let mut rng = rand::thread_rng();
    let mut r = n;
    while gcd(r, n) != 1 {
        r = rng.gen_range(1..n);
    }

    // calculating the encryption
    let k1 = mod_exp(g, m, n.pow(2));   // g^M mod N^2
    let k2 = mod_exp(r, n, n.pow(2));   // r^N mod N^2
    let c_key = (k1 * k2) % n.pow(2);   // (g^M * r^N) mod N^2

    Ok(c_key)
}

// modular exponentiation function for large numbers
pub fn mod_exp(base: u128, exp: u128, modulus: u128) -> u128 {
    let mut result = 1;
    let mut base = base % modulus;
    let mut exp = exp;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp >>= 1;
        base = (base * base) % modulus;
    }

    result
}

pub fn decipher_paillier(private_key: (u128, u128), c_key: u128, public_key: (u128, u128)) -> u128 {
    let (lambda, mu) = private_key;
    let (g, n) = public_key;

    // function L = (x - 1) / N
    let l = |x: u128| -> u128 { (x - 1) / n };

    // L(c^λ mod N^2)
    let l_value = l(mod_exp(c_key, lambda, n.pow(2)));

    // M = L(c^λ mod N^2) * μ mod N
    let m = (l_value * mu) % n;
    
    m
}
