use mpc_zk_paillier::curve::ecc;

fn main() {

    let new_point = ecc::Point::new(10, 16);
    let new_ec = ecc::EcWei::new(-2, 7, 17);
    println!("The {:?} belongs to the curve ?: {:?}", new_point, new_ec.is_point(&new_point));

    let group_add = ecc::EcWei::group_points(&new_ec);
    println!("The elements of the curve group:\n {:?}", group_add);
    ecc::EcWei::cayley_table(&new_ec, &group_add);
    //println!("The elements of the curve group:\n {:?}", group_table);


}
