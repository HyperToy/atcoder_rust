use proconio::*;
use rand::Rng;

fn main() {
    input! {
        (n, k, l): (usize, usize, i32),
        ab: [(i32, i32); k],
        c: [[usize; n]; n],
    }
    let mut rng = rand::thread_rng();
    for _ in 0..k {
        println!("{}", rng.gen_range(0..l) + 1);
    }
}
