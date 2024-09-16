use mpc_zk_paillier::curve::ecc;

fn main() {

    let new_point = ecc::Point::new(10, 16);

    let new_ec = ecc::EcWei::new(-2, 7, 17);
    println!("The {:?} belongs to the curve ?: {:?}", new_point, new_ec.is_point(&new_point));

    let p1 = ecc::Point { x: 13, y: 11 };
    let p2 = ecc::Point { x: 9, y: 2 };

    let result_add = new_ec.point_addition(p1, p2);
    println!("Sum of points: {:?}", result_add);

}
