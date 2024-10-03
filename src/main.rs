use mpc_zk_paillier::{curve::ecc::*, mpc_wallet};
use num_bigint::BigInt;
use num_traits::FromPrimitive;


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

    mpc_wallet::ecdsa_mpc(&new_ec, &group_add, &points_g);

}
