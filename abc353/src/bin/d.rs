use proconio::input;
const MOD: u64 = 998_244_353;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }
    let mut digit_count = [0; 11];
    let digit_value = [
        1,
        10,
        100,
        1_000,
        10_000,
        100_000,
        1_000_000,
        10_000_000,
        100_000_000,
        1_000_000_000 % MOD,
        10_000_000_000 % MOD,
    ];
    let mut answer = 0;
    for i in (0..n).rev() {
        for d in 0..11 {
            answer += digit_count[d] * digit_value[d] % MOD * a[i] % MOD;
            answer %= MOD;
        }
        answer += a[i] * (i as u64) % MOD;
        answer %= MOD;
        digit_count[digit(a[i])] += 1;
    }
    println!("{}", answer);
}
fn digit(x: u64) -> usize {
    x.to_string().len()
}
