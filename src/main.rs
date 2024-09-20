use mpc_zk_paillier::curve::ecc::*;

fn main() {
    let new_ec = EcWei::new(-3, 4, 17);
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

    let point_g = new_ec.get_base_points(&group_add);
    println!("Generator points {:?}", point_g);
}
