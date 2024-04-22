use proconio::input;

const MOD: u64 = 998244353;

fn main() {
    input! {
        n: usize, p: u64,
    }
    let mut dp = vec![0; n + 1];
    dp[0] = 0;
    dp[1] = 1;
    let p = p * mod_inverse(100) % MOD;
    for i in 2..=n {
        dp[i] = ((MOD + 1 - p) * (1 + dp[i - 1]) + p * (1 + dp[i - 2])) % MOD;
    }
    println!("{}", dp[n]);
}
fn mod_inverse(a: u64) -> u64 {
    pow(a, MOD - 2)
}
// repeated square 繰返し二乗法
fn pow(mut a: u64, mut n: u64) -> u64 {
    let mut res = 1;
    while n > 0 {
        if n % 2 > 0 {
            res *= a;
            res %= MOD;
        }
        a *= a;
        a %= MOD;
        n /= 2;
    }
    res
}
