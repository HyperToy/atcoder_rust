use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize, q: usize,
        t: [Usize1; q],
    }
    let mut teeth = vec![true; n];
    for x in t {
        teeth[x] ^= true;
    }
    println!("{}", teeth.iter().filter(|x| **x).count());
}
