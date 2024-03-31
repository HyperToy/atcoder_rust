use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        a: [i32; 3],
    }
    println!(
        "{}",
        a.iter()
            .permutations(3)
            .map(|a| (a[0] * 10 + a[1]) + a[2])
            .max()
            .unwrap()
    );
}
