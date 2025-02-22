use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize, x: usize,
        t: [usize; n],
    }
    let mut p = vec![0; x + 1];
    p[0] = 1;
    let inv_n = mod_inverse(n);
    for i in 0..x {
        for k in 0..n {
            if i + t[k] > x {
                continue;
            }
            // i + t[k] への遷移
            p[i + t[k]] += p[i] * inv_n;
            p[i + t[k]] %= MOD;
        }
    }

    let mut answer = 0;
    let start = if x <= t[0] { 0 } else { x - t[0] + 1 };
    for i in start..=x {
        answer += p[i];
        answer %= MOD;
    }
    answer *= inv_n;
    answer %= MOD;
    println!("{}", answer);
}

fn mod_inverse(a: usize) -> usize {
    pow(a, MOD - 2)
}
fn pow(mut a: usize, mut n: usize) -> usize {
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

// TODO: modint 導入
