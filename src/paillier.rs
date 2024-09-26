use crate::arithmetic::basic_op;
extern crate rand;
use rand::Rng;

#[derive(Debug)]
pub struct PaillierKey {
    pub public_key: (i64, i64),
    pub private_key: (i64, i64),
}

pub fn gen_key_paillier(p: i64, q: i64) -> PaillierKey {
    // calculate N
    let n = p * q;
    // calculate λ
    let lambda = basic_op::lcm(p - 1, q - 1);

    // generate g randomly
    let mut rng = rand::thread_rng();
    let mut g = n;
    let mut l: i64 = n;
    
    // select g and calculate L until gcd(L, N) == 1
    while basic_op::gcd(l, n) != 1 {
        g = rng.gen_range(1..n.pow(2));
        l = ((g.pow(lambda as u32) % n.pow(2)) - 1) / n;
    }

    // calculate μ using the modular inverse
    let mu = basic_op::inv_mod(l, n);

    PaillierKey {
        public_key: (g, n),
        private_key: (lambda, mu.unwrap()),
    }
}

pub fn cipher_paillier(public_key: (i64, i64), m: i64) -> Result<i64, &'static str> {
    let (g, n) = public_key;

    // ensure that the M message is appropriate
    if m >= n || m <= 0 {
        return Err("El mensaje o secreto no es apropiado");
    }

    // generate r randomly, making sure that gcd(r, N) == 1
    let mut rng = rand::thread_rng();
    let mut r = n;
    while basic_op::gcd(r, n) != 1 {
        r = rng.gen_range(1..n);
    }

    // calculating the encryption
    let k1 = mod_exp(g, m, n.pow(2));   // g^M mod N^2
    let k2 = mod_exp(r, n, n.pow(2));   // r^N mod N^2
    let c_key = (k1 * k2) % n.pow(2);   // (g^M * r^N) mod N^2

    Ok(c_key)
}

// modular exponentiation function for large numbers
pub fn mod_exp(base: i64, exp: i64, modulus: i64) -> i64 {
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

pub fn decipher_paillier(private_key: (i64, i64), c_key: i64, public_key: (i64, i64)) -> i64 {
    let (lambda, mu) = private_key;
    let (_g, n) = public_key;

    // function L = (x - 1) / N
    let l = |x: i64| -> i64 { (x - 1) / n };

    // L(c^λ mod N^2)
    let l_value = l(mod_exp(c_key, lambda, n.pow(2)));

    // M = L(c^λ mod N^2) * μ mod N
    let m = (l_value * mu) % n;
    
    m
}
