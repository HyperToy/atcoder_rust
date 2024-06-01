use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let a = a.into_iter().sorted().collect::<Vec<_>>();
}
