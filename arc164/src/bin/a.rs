use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        t: usize,
        cases: [(u64, u64); t],
    }
    println!(
        "{}",
        cases
            .into_iter()
            .map(|(n, k)| (n - k) % 2 == 0 && k >= digit_sum(n))
            .map(|b| if b { "Yes" } else { "No" })
            .join("\n")
    );
}
fn digit_sum(mut n: u64) -> u64 {
    let mut res = 0;
    while n > 0 {
        res += n % 3;
        n /= 3;
    }
    res
}
