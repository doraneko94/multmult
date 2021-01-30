use multmult::*;

fn main() {
    let mut mm = MultMult::<f64>::new();
    mm.mul_fact(5);
    mm.div_fact(5);
    mm.mul_fact(7);
    mm.div_fact(6);
    mm.mul_comb(5, 2);
    mm.div_perm(9, 2);
    println!("{}", mm.value());
}