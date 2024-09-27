use mpc_zk_paillier::curve::ecc::*;
use mpc_zk_paillier::paillier::*;
use num_bigint::BigInt; // Importamos BigInt
use num_traits::FromPrimitive; // Importamos FromPrimitive para convertir enteros a BigInt

fn main() {
    // Convertimos los enteros a BigInt
    let new_ec: EcWei = EcWei::new(
        BigInt::from_i64(-2).unwrap(),
        BigInt::from_i64(7).unwrap(),
        BigInt::from_i64(17).unwrap(),
    );

    //println!("The {:?} belongs to the curve ?: {:?}", new_point, new_ec.is_point(&new_point));
    println!("{:?}", new_ec.is_point(&Point::new(
        BigInt::from_i64(4).unwrap(),
        BigInt::from_i64(7).unwrap()
    )));

    let group_add = new_ec.group_points();
    new_ec.cayley_table(&group_add);
    println!(
        "{:?}",
        new_ec.point_add(
            &Point::new(BigInt::from_i64(8).unwrap(), BigInt::from_i64(4).unwrap()),
            &Point::new(BigInt::from_i64(8).unwrap(), BigInt::from_i64(4).unwrap())
        )
    ); // ()
    println!(
        "{:?}",
        new_ec.point_add(
            &Point::new(BigInt::from_i64(8).unwrap(), BigInt::from_i64(4).unwrap()),
            &Point::new(BigInt::from_i64(6).unwrap(), BigInt::from_i64(7).unwrap())
        )
    );

    let point_a = group_add[5].clone();
    let point_b = group_add[2].clone();
    let sum_point = new_ec.point_add(&point_a, &point_b);

    println!(
        "\nThe sum of the point {} with the point {} is: {} ",
        point_a.to_string(),
        point_b.to_string(),
        sum_point.to_string(),
    );

    // Generating point G and non-generating points
    let point_g = new_ec.get_base_points(&group_add);
    println!("Generator points {:?}", point_g);

    let other_ec = EcWei::new(
        BigInt::from_i64(-3).unwrap(),
        BigInt::from_i64(4).unwrap(),
        BigInt::from_i64(17).unwrap(),
    );
    let other_group_add = other_ec.group_points();
    other_ec.cayley_table(&other_group_add);
    println!(
        "{:?}",
        other_ec.scalar_mul(&Point::new(BigInt::from_i64(6).unwrap(), BigInt::from_i64(10).unwrap()), 2)
    ); // (6,7)
    println!(
        "{:?}",
        other_ec.scalar_mul(&Point::new(BigInt::from_i64(6).unwrap(), BigInt::from_i64(10).unwrap()), 3)
    ); // (0,0) = ∞
    println!(
        "{:?}",
        other_ec.scalar_mul(&Point::new(BigInt::from_i64(6).unwrap(), BigInt::from_i64(10).unwrap()), 4)
    ); // (6,10)
    println!(
        "{:?}",
        other_ec.scalar_mul(&Point::new(BigInt::from_i64(6).unwrap(), BigInt::from_i64(10).unwrap()), 5)
    ); // (6,7)
    println!(
        "{:?}",
        other_ec.scalar_mul(&Point::new(BigInt::from_i64(6).unwrap(), BigInt::from_i64(10).unwrap()), 6)
    ); // (0,0)
    println!(
        "{:?}",
        other_ec.scalar_mul(&Point::new(BigInt::from_i64(6).unwrap(), BigInt::from_i64(10).unwrap()), 7)
    ); // (6,10) = ∞

    // TSS setup with ECDSA: For two parties
    let generators = new_ec.get_base_points(&group_add);
    let point_g = generators[0].clone();
    let key_pair_1 = new_ec.gen_key_pair(&point_g);
    println!(
        " Generator {:?} -> {:?}",
        point_g,
        &key_pair_1.as_ref().unwrap()
    );
    let key_pair_2 = new_ec.gen_key_pair(&point_g);
    println!(
        " Generator {:?} -> {:?}",
        point_g,
        &key_pair_2.as_ref().unwrap()
    );

    // Diffie-Hellman
    println!(" Part 1 computa Q = Q_2 * d1:");
    let part_1_dh = new_ec.scalar_mul(
        &key_pair_2.as_ref().unwrap().pk,
        key_pair_1.as_ref().unwrap().sk,
    );
    println!(" Part 1 gets Q as -> {:?}", &part_1_dh);
    println!(" Part 2 computa Q = Q_1 * d2:");
    let part_2_dh = new_ec.scalar_mul(
        &key_pair_1.as_ref().unwrap().pk,
        key_pair_2.as_ref().unwrap().sk,
    );
    println!(" Part 1 gets Q as -> {:?}", &part_2_dh);
    println!(
        " The Diffie-Hellman protocol is followed -> {:?}",
        &part_1_dh == &part_2_dh
    );

    // Paillier key generation
    let paillier_key_p1 = gen_key_paillier(BigInt::from_i64(11).unwrap(), BigInt::from_i64(3).unwrap());
    println!(" Part 1 -> {:?}", &paillier_key_p1);
}
