use proconio::input;

fn main() {
    input! {
        n: i64, m: i64, k: i64,
    }
    let lcm = n * m / gcd(n, m);
    let mut ok = 2_000_000_000_000_000_000;
    let mut ng = 0;
    while ng + 1 < ok {
        let wj = (ok + ng) / 2;
        if count(n, m, lcm, wj) >= k {
            ok = wj;
        } else {
            ng = wj;
        }
    }
    println!("{}", ok);
}
fn gcd(a: i64, b: i64) -> i64 {
    if b % a == 0 {
        a
    } else if a > b {
        gcd(b, a)
    } else {
        gcd(b % a, a)
    }
}
fn count(n: i64, m: i64, l: i64, x: i64) -> i64 {
    x / n + x / m - x / l * 2
}

#[cfg(test)]
mod test {
    use crate::gcd;
    #[test]
    fn check() {
        assert_eq!(gcd(24, 18), 6);
        assert_eq!(gcd(7, 3), 1);
    }
}
