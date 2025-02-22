use ac_library::ModInt998244353 as Mint;
use proconio::input;

fn main() {
    input! {
        n: u64, k: u64,
    }
    // 確率漸化式を解いて期待値を求めた。
    let a = (Mint::from(n - 2) / n).pow(k);
    let answer = a + Mint::from(n + 1) / 2 * (Mint::from(1) - a);
    println!("{}", answer);
}
