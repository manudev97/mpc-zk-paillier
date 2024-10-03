use mpc_zk_paillier::arithmetic::basic_op::inv_mod;
use mpc_zk_paillier::{arithmetic::basic_op, curve::ecc::*};
use mpc_zk_paillier::paillier::*;
use sha2::{Digest, Sha256};
use num_bigint::BigInt;
use rand::Rng;
use num_traits::{FromPrimitive, One, Zero};

fn main() {
    // Convert integers to BigInt
    let new_ec: EcWei = EcWei::new(
        BigInt::from_i64(-2).unwrap(),
        BigInt::from_i64(7).unwrap(),
        BigInt::from_i64(17).unwrap(),
    );

    //println!("The {:?} belongs to the curve ?: {:?}", new_point, new_ec.is_point(&new_point));
    println!(
        "{:?}",
        new_ec.is_point(&Point::new(
            BigInt::from_i64(4).unwrap(),
            BigInt::from_i64(7).unwrap()
        ))
    );

    let group_add = new_ec.group_points();
    new_ec.cayley_table(&group_add);

    let point_a = group_add[5].clone();
    let point_b = group_add[2].clone();
    let sum_point = new_ec.point_add(&point_a, &point_b);

    println!(
        "\nThe sum of the point {} with the point {} is: {} \n",
        point_a.to_string(),
        point_b.to_string(),
        sum_point.to_string(),
    );

    // Generating point G and non-generating points
    let points_g = new_ec.get_base_points(&group_add);
    for point in points_g.iter() {
        println!("Generator {:?}", point);
    }

    let other_ec = EcWei::new(
        BigInt::from_i64(-3).unwrap(),
        BigInt::from_i64(4).unwrap(),
        BigInt::from_i64(17).unwrap(),
    );

    let other_group_add = other_ec.group_points();
    other_ec.cayley_table(&other_group_add);

    println!("\n ----+------ The point G = (6,10) is not generator: ----+------\n");
    println!(
        " G * 2 = {:?}",
        other_ec.scalar_mul(
            &Point::new(BigInt::from_i64(6).unwrap(), BigInt::from_i64(10).unwrap()),
            &BigInt::from(2)
        )
    ); // (6,7)
    println!(
        " G * 3 = {:?}",
        other_ec.scalar_mul(
            &Point::new(BigInt::from_i64(6).unwrap(), BigInt::from_i64(10).unwrap()),
            &BigInt::from(3)
        )
    ); // (0,0) = ∞
    println!(
        " G * 4 = {:?}",
        other_ec.scalar_mul(
            &Point::new(BigInt::from_i64(6).unwrap(), BigInt::from_i64(10).unwrap()),
            &BigInt::from(4)
        )
    ); // (6,10)
    println!(
        " G * 5 = {:?}",
        other_ec.scalar_mul(
            &Point::new(BigInt::from_i64(6).unwrap(), BigInt::from_i64(10).unwrap()),
            &BigInt::from(5)
        )
    ); // (6,7)
    println!(
        " G * 6 ={:?}",
        other_ec.scalar_mul(
            &Point::new(BigInt::from_i64(6).unwrap(), BigInt::from_i64(10).unwrap()),
            &BigInt::from(6)
        )
    ); // (0,0)
    println!(
        " G * 7 = {:?}",
        other_ec.scalar_mul(
            &Point::new(BigInt::from_i64(6).unwrap(), BigInt::from_i64(10).unwrap()),
            &BigInt::from(7)
        )
    ); // (6,10) = ∞

    // TSS setup with ECDSA: For two parties
    println!("\n ----+------ TSS setup with ECDSA: For two parties: ----+------ \n");
    let generators = new_ec.get_base_points(&group_add);
    let point_g = generators[0].clone();
    let key_pair_1 = new_ec.gen_key_pair(&point_g);
    println!(
        " Generator {:?} -> {:?} Part 1",
        point_g,
        &key_pair_1.as_ref().unwrap()
    );
    let key_pair_2 = new_ec.gen_key_pair(&point_g);
    println!(
        " Generator {:?} -> {:?} Part 2",
        point_g,
        &key_pair_2.as_ref().unwrap()
    );

    // Diffie-Hellman
    println!("\n ----+------ Diffie-Hellman (DH): ----+------ \n");
    println!(" Part 1 computa Q = Q_2 * d1:");
    let part_1_dh = new_ec.scalar_mul(
        &key_pair_2.as_ref().unwrap().pk,
        &key_pair_1.as_ref().unwrap().sk,
    );
    println!(" Part 1 gets Q as -> {:?}", &part_1_dh);
    println!(" Part 2 computa Q = Q_1 * d2:");
    let part_2_dh = new_ec.scalar_mul(
        &key_pair_1.as_ref().unwrap().pk,
        &key_pair_2.as_ref().unwrap().sk,
    );
    println!(" Part 1 gets Q as -> {:?}", &part_2_dh);
    println!(
        " The Diffie-Hellman protocol is followed -> {:?}",
        &part_1_dh == &part_2_dh
    );
    let mut point_g = Point::new(BigInt::zero(),BigInt::zero());
    if part_1_dh == part_2_dh {
        point_g = part_1_dh;
    }

    // Paillier key generation
    println!("\n ----+------ Paillier key generation: ----+------ \n");
    let paillier_key_p1 = gen_key_paillier(
        &BigInt::from_i64(11).unwrap(),
        &BigInt::from_i64(3).unwrap(),
    );
    println!(" Part 1 -> {:?}", &paillier_key_p1);

    // Chiper secret with Paillier - Part 1
    let chiper_p1 = cipher_paillier(
        &paillier_key_p1.public_key,
        &key_pair_1.as_ref().unwrap().sk,
    );
    let dechiper_p1 = decipher_paillier(
        &paillier_key_p1.private_key,
        chiper_p1.as_ref().unwrap().clone(),
        &paillier_key_p1.public_key,
    );
    println!(
        " Encrypting the secret ({:?}) of Part 1 -> {:?}",
        &key_pair_1.as_ref().unwrap().sk,
        &chiper_p1.as_ref().unwrap()
    );
    println!(
        " Decrypting chipher ({:?}) -> {:?}",
        &chiper_p1.as_ref().unwrap(),
        &dechiper_p1
    );

    // Chiper secret with Paillier - Part 2
    let chiper_p2 = cipher_paillier(
        &paillier_key_p1.public_key,
        &key_pair_2.as_ref().unwrap().sk,
    );
    println!(
        " Encrypting the secret ({:?}) of Part 2 -> {:?}",
        &key_pair_2.as_ref().unwrap().sk,
        &chiper_p2.as_ref().unwrap()
    );

    // Paillier Homomorphic
    println!("\n ----+------ Check homomorphism ----+------\n");
    println!("       Dec(Enc(m_1) * Enc(m_2)) = m_1 + m_2");
    println!(
        "       Dec(Enc({0:?}) * Enc({1:?})) = {0:?} + {1:?}",
        &key_pair_1.as_ref().unwrap().sk,
        &key_pair_2.as_ref().unwrap().sk
    );
    println!(
        "       Dec({0:?} * {1:?}) = {2:?} + {3:?}",
        &chiper_p1.as_ref().unwrap(),
        &chiper_p2.as_ref().unwrap(),
        &key_pair_1.as_ref().unwrap().sk,
        &key_pair_2.as_ref().unwrap().sk
    );
    println!(
        "       Dec({0:?}) = {1:?} + {2:?}",
        chiper_p1.as_ref().unwrap() * chiper_p2.as_ref().unwrap(),
        &key_pair_1.as_ref().unwrap().sk,
        &key_pair_2.as_ref().unwrap().sk
    );
    println!(
        "       Dec({0:?}) = {1:?} + {2:?}",
        (chiper_p1.as_ref().unwrap() * chiper_p2.as_ref().unwrap())
            % &paillier_key_p1.public_key.1.clone().pow(2),
        &key_pair_1.as_ref().unwrap().sk,
        &key_pair_2.as_ref().unwrap().sk
    );
    println!(
        "Part 1 Dec:  {0:?} = {1:?} + {2:?}",
        decipher_paillier(
            &paillier_key_p1.private_key,
            (chiper_p1.as_ref().unwrap() * chiper_p2.as_ref().unwrap())
                % &paillier_key_p1.public_key.1.clone().pow(2),
            &paillier_key_p1.public_key
        ),
        &key_pair_1.as_ref().unwrap().sk,
        &key_pair_2.as_ref().unwrap().sk
    );

    // MPC Wallet 
    println!("\n ----+------ MPC Wallet ----+------\n");
    println!(" MPC wallet will sign the message M");
    let message = "Hello Victor, this is a message from Peggy";
    println!(" (M = {})", message);

    println!("\n ----+------ MPC Wallet Configuration ----+------");
    println!("\n   + --- Part 1 generates a random secret k1, point R1 and a ZK proof --- +  \n");
    let ckey = chiper_p1;
    let mut rng = rand::thread_rng();
    let k1 = BigInt::from(rng.gen_range(1..(group_add.len() + 1)));
    println!("     k1 = {}", &k1);
    let point_r1 = new_ec.scalar_mul(&points_g[0], &k1); 
    println!("     R1 = {:?}", &point_r1);
    println!("     => Post R1 and ZK proof that it correctly generated k1");
    println!("\n   + --- Part 2 generates a random secret k2, point R2 and a ZK proof --- +  \n");
    let k2 = BigInt::from(rng.gen_range(2..(group_add.len() + 1)));
    println!("     k2 = {}", &k2);
    let point_r2 = new_ec.scalar_mul(&points_g[0], &k2);
    println!("     R2 = {:?}", &point_r2);
    println!("     => Post R2 and ZK proof that it correctly generated k2");
    println!("\n   + --- Through DH they secretly share an R point --- + \n");
    println!("     Shared secret Parte 1 (R = {:?})", new_ec.scalar_mul(&point_r2, &k1));
    println!("     Shared secret Parte 2 (R = {:?})", new_ec.scalar_mul(&point_r1, &k2));

    println!("\n ----+------ MPC Signature ----+------");
    println!("\n   + --- Part 2 operates homomorphically --- + \n");
    let mut hash_message_p2 = Sha256::new();
    hash_message_p2.update(message.as_bytes());
    let hash_result_p2 = hash_message_p2.finalize();
    let hash_message_p2_to_ec = BigInt::from_bytes_be(num_bigint::Sign::Plus, &hash_result_p2) % BigInt::from(15);
    println!("    [H(M) = {:?}]", &hash_message_p2_to_ec);
    let mut rho = paillier_key_p1.public_key.1.clone();
    while basic_op::gcd(&rho, &paillier_key_p1.public_key.1.clone()) != BigInt::from(1) {
        rho = BigInt::from(rng.gen_range(0..(group_add.len() + 1).pow(2)));
    }
    let inv_k2 = inv_mod(&k2, &BigInt::from(group_add.len() + 1));
    let c1 = cipher_paillier(
        &paillier_key_p1.public_key,
        &((&rho * &BigInt::from(group_add.len() + 1) + inv_k2.as_ref().unwrap() * &hash_message_p2_to_ec) % BigInt::from(group_add.len() + 1))
    );
    println!("    c1 = Enc(N + k2^-1 * H(M)) = {:?}", c1.as_ref().unwrap());
    let v = (inv_k2.as_ref().unwrap() * &new_ec.scalar_mul(&point_r1, &k2).x * &key_pair_2.as_ref().unwrap().sk) % BigInt::from(group_add.len() + 1);
    let c2 = ckey.as_ref().unwrap().modpow(&v, &paillier_key_p1.public_key.1.pow(2));
    println!("    c2 = k2^-1 * r * d2 * cKey = {:?}", &c2);
    let c3 = (c1.as_ref().unwrap() * &c2).modpow(&BigInt::one(), &paillier_key_p1.public_key.1.pow(2));
    println!("    c3 = c1 + c2 = {:?}", &c3);
    println!("    => Part 2 send (c3 = {:?}) to Part 1", &c3);

    println!("\n   + --- Part 1 compute firma r y s --- + \n");
    let mut s = decipher_paillier(&paillier_key_p1.private_key, c3, &paillier_key_p1.public_key);
    let r = new_ec.scalar_mul(&point_r2, &k1).x % BigInt::from(group_add.len() + 1);
    s = (s * inv_mod(&k1, &BigInt::from(group_add.len() + 1)).unwrap()) % BigInt::from(group_add.len() + 1);
    println!("    => Part 1 post the signature like (r, s): ({:?}, {:?})", &r, &s);

    println!("\n   + --- The MPC wallet verifies the signature ({:?}, {:?}) for the message --- + \n",&r, &s);
    let mut hash_message_verifier = Sha256::new();
    hash_message_verifier.update(message.as_bytes());
    let hash_result_verifier = hash_message_verifier.finalize();
    let hash_message_verifier_to_ec = BigInt::from_bytes_be(num_bigint::Sign::Plus, &hash_result_verifier) % BigInt::from(15);
    println!("    [H({:?}) = {:?}]", &message, &hash_message_verifier_to_ec);
    let u1 = (hash_message_verifier_to_ec * inv_mod(&s, &BigInt::from(group_add.len() + 1)).unwrap()) % BigInt::from(group_add.len() + 1);
    let u2 = (&r * inv_mod(&s, &BigInt::from(group_add.len() + 1)).unwrap()) % BigInt::from(group_add.len() + 1);
    let x = new_ec.point_add(&new_ec.scalar_mul(&points_g[0], &u1), &new_ec.scalar_mul(&point_g, &u2)).x;
    if &r == &(x % BigInt::from(group_add.len() + 1)) {
        println!("    The signature ({:?}, {:?}) is correct...", &r, &s )
    } else {
        println!("    The signature ({:?}, {:?}) is incorrect...", &r, &s )
    }
}
