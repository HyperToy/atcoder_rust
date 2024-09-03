use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    println!(
        "{}",
        a.into_iter()
            .counts()
            .into_iter()
            .map(|(_, v)| v / 2)
            .sum::<usize>()
    );
}
