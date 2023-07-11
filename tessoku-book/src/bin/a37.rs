use proconio::*;

fn main() {
    input! {
        n: u64, m: u64, b: u64,
        a: [u64; n],
        c: [u64; m],
    }
    println!(
        "{}",
        m * a.iter().sum::<u64>() + n * c.iter().sum::<u64>() + b * n * m
    );
}
