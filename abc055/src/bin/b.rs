use proconio::*;

fn main() {
    input! {
        n: u64,
    }
    let mut ans: u64 = 1;
    let m: u64 = 1_000_000_000 + 7;
    for i in 1..=n {
        ans *= i;
        ans %= m;
    }
    println!("{}", ans);
}
