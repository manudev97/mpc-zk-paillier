use mpc_zk_paillier::curve::ecc;
fn main() {

    let new_point = ecc::Point::new(10, 16);

    let new_ec = ecc::EcWei::new(-2, 7, 17);
    println!("The {:?} belongs to the curve ?: {:?}", new_point, new_ec.is_point(&new_point));
    
}
