use proconio::input;
use std::convert::TryInto;

fn main() {
    input! {
        a: [u64; 64],
    }
    println!(
        "{}",
        a.iter()
            .enumerate()
            .map(|(i, x)| x * 2u64.pow(i.try_into().unwrap()))
            .sum::<u64>()
    );
}
