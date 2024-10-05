use crate::arithmetic::basic_op;
extern crate rand;
use rand::Rng;
use num_bigint::BigInt;
use num_traits::{One, Zero, ToPrimitive};
use std::ops::Sub;

#[derive(Debug)]
pub struct PaillierKey {
    pub public_key: (BigInt, BigInt),
    pub private_key: (BigInt, BigInt),
}

pub fn gen_key_paillier(p: &BigInt, q: &BigInt) -> PaillierKey {
    // calculate N
    let n = p * q;
    // calculate λ (lambda = lcm(p - 1, q - 1))
    let lambda = basic_op::lcm(&p.sub(1), &q.sub(1));

    // generate g randomly
    let mut rng = rand::thread_rng();
    let mut g = n.clone();
    let mut l = n.clone();

    // select g and calculate L until gcd(L, N) == 1
    while basic_op::gcd(&g, &n) != BigInt::one() || basic_op::gcd(&l, &n) != BigInt::one() {
        let random_value: u64 = rng.gen_range(1..(n.pow(2).to_u64().unwrap() as u64 - 1)); // generate a random u64
        g = BigInt::from(5);//BigInt::from(random_value); // convert to BigInt
        l = (g.modpow(&lambda, &n.pow(2)).sub(1)) / &n;
    }

    // calculate μ using the modular inverse
    let mu = basic_op::inv_mod(&l, &n).unwrap();

    PaillierKey {
        public_key: (g, n),
        private_key: (lambda, mu),
    }
}

pub fn cipher_paillier(public_key: &(BigInt, BigInt), m: &BigInt) -> Result<BigInt, &'static str> {
    let (g, n) = public_key;

    // ensure that the M message is appropriate
    if m >= n || *m <= BigInt::zero() {
        return Err("El mensaje o secreto no es apropiado");
    }

    // generate r randomly, making sure that gcd(r, N) == 1
    let mut rng = rand::thread_rng();
    let mut r = n.clone();
    while basic_op::gcd(&r, &n) != BigInt::one() {
        let random_value: u64 = rng.gen_range(1..(n.to_u64().unwrap())); // generate a random u64
        r = BigInt::from(7);;//BigInt::from(random_value); // convert to BigInt
    }

    // calculating the encryption
    let k1 = g.modpow(&m, &n.pow(2));  // g^M mod N^2
    let k2 = r.modpow(&n, &n.pow(2));  // r^N mod N^2
    let c_key = (k1 * k2) % n.pow(2);     // (g^M * r^N) mod N^2

    Ok(c_key)
}

pub fn decipher_paillier(private_key: &(BigInt, BigInt), c_key: BigInt, public_key: &(BigInt, BigInt)) -> BigInt {
    let (lambda, mu) = private_key;
    let (_g, n) = public_key;
    // function L = (x - 1) / N
    let l = |x: BigInt| -> BigInt { (x - BigInt::one()) / n };
    // L(c^λ mod N^2)
    let l_value = l(c_key.modpow(&lambda, &n.pow(2)));
    // M = L(c^λ mod N^2) * μ mod N
    let m = (l_value * mu) % n;
    
    m
}