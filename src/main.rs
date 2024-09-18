use mpc_zk_paillier::curve::ecc::*;

fn main() {

    
    let new_ec = EcWei::new(-2, 7, 17);
    //println!("The {:?} belongs to the curve ?: {:?}", new_point, new_ec.is_point(&new_point));

    let group_add = EcWei::group_points(&new_ec);
    EcWei::cayley_table(&new_ec, &group_add);
    
    let point_a = group_add[5]; 
    let point_b = group_add[2];
    let sum_point = EcWei::point_add(&new_ec, &point_a, &point_b);

    println!("\nThe sum of the {point_a:?} with the {point_b:?} is: {sum_point:?} ", );


}
