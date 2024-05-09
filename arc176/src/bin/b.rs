use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        t: usize,
        queries: [(u64, u64, u64); t],
    }
    println!(
        "{}",
        queries
            .into_iter()
            .map(|(n, m, k)| solve(n, m, k))
            .join("\n")
    );
}

fn solve(n: u64, m: u64, k: u64) -> u64 {
    let cycle = m - k;
    let offset = k - 1;
    let n = if n > offset {
        (n - offset - 1) % cycle + 1 + offset
    } else {
        n
    };
    if cycle == 1 && n > offset {
        0
    } else {
        pow(2, n)
    }
}

// repeated square 繰返し二乗法
fn pow(mut a: u64, mut n: u64) -> u64 {
    let mut res = 1;
    while n > 0 {
        if n % 2 > 0 {
            res *= a;
            res %= 10;
        }
        a *= a;
        a %= 10;
        n /= 2;
    }
    res
}
