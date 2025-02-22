use proconio::input;
const MOD: u64 = 998244353;

fn main() {
    input! {
        a: u64, b: u64,
    }
    let b_is_odd = b % 2 == 1;
    let c_is_odd = !b_is_odd || is_square(a);
    let b = b % MOD;
    let pf = prime_factorize(a);
    let mut c = 1;
    for (_, ex) in pf {
        c *= (b * ex % MOD + 1) % MOD;
        c %= MOD;
    }
    println!(
        "{}",
        if c_is_odd {
            (b * (c + MOD - 1) % MOD * mod_inverse(2) % MOD
                + if b_is_odd { b - 1 + MOD } else { b } * mod_inverse(2) % MOD)
                % MOD
        } else {
            b * c % MOD * mod_inverse(2) % MOD
        }
    );
}

// todo snippet
fn prime_factorize(mut n: u64) -> Vec<(u64, u64)> {
    let mut res = Vec::new();
    for a in 2.. {
        if a * a > n {
            break;
        }
        if n % a != 0 {
            continue;
        }
        let mut ex = 0;
        while n % a == 0 {
            ex += 1;
            n /= a;
        }
        res.push((a, ex));
    }
    if n != 1 {
        res.push((n, 1))
    };
    res
}
fn is_square(n: u64) -> bool {
    for a in 2.. {
        if a * a > n {
            break;
        }
        if a * a == n {
            return true;
        }
    }
    false
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
