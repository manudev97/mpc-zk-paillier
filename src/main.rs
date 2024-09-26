use mpc_zk_paillier::curve::ecc::*;
use mpc_zk_paillier::paillier::*;
fn main() {
    let new_ec: EcWei = EcWei::new(-2, 7, 17);
    //println!("The {:?} belongs to the curve ?: {:?}", new_point, new_ec.is_point(&new_point));
    println!("{:?}", new_ec.is_point(&Point::new(4, 7)));

    let group_add = new_ec.group_points();
    new_ec.cayley_table(&group_add);
    println!(
        "{:?}",
        new_ec.point_add(&Point::new(8, 4), &Point::new(8, 4))
    ); // ()
    println!(
        "{:?}",
        new_ec.point_add(&Point::new(8, 4), &Point::new(6, 7))
    );

    let point_a = group_add[5];
    let point_b = group_add[2];
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

    let other_ec = EcWei::new(-3, 4, 17);
    let other_group_add = other_ec.group_points();
    other_ec.cayley_table(&other_group_add);
    println!("{:?}", other_ec.scalar_mul(&Point::new(6, 10), 2)); // (6,7)
    println!("{:?}", other_ec.scalar_mul(&Point::new(6, 10), 3)); // (0,0) = ∞
    println!("{:?}", other_ec.scalar_mul(&Point::new(6, 10), 4)); // (6,10)
    println!("{:?}", other_ec.scalar_mul(&Point::new(6, 10), 5)); // (6,7)
    println!("{:?}", other_ec.scalar_mul(&Point::new(6, 10), 6)); // (0,0)
    println!("{:?}", other_ec.scalar_mul(&Point::new(6, 10), 7)); // (6,10) = ∞

    // TSS setup with ECDSA: For two parties
    let generators = new_ec.get_base_points(&group_add);
    let point_g = generators[0];
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
    // let paillier_key_p1 = gen_key_paillier(11 ,3);
    // println!(" Part 1 Paillier Keys -> {:?}", &paillier_key_p1);
    
}
