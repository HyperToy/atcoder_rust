use proconio::*;

fn main() {
    input! {
        t: usize,
    }
    const MOD: u64 = 998244353;
    for _ in 0..t {
        input! {
            n: u64,
        }
        let mut ans = 0;
        for y in 1..=n {
            if y.pow(2) > n {
                break;
            }
            ans += 6 * (y - 1) * (n / y - y) + 3 * (n / y - y) + 3 * (y - 1) + 1;
            ans %= MOD;
        }
        println!("{}", ans);
    }
}
