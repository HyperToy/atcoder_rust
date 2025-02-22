use proconio::{input, marker::Bytes};

fn main() {
    input! {
        n: usize,
        mut s: Bytes
    }
    s.sort();
    println!(
        "{}",
        (0..10_000_000i64)
            .map(|x| x * x)
            .filter(|x| *x < 10i64.pow(n as u32))
            .map(|mut x| {
                let mut res = Vec::new();
                while x > 0 {
                    res.push((x % 10) as u8 + b'0');
                    x /= 10;
                }
                while res.len() < n {
                    res.push(b'0');
                }
                res.sort();
                res
            })
            .filter(|v| v.len() == n)
            .filter(|v| *v == s)
            .count()
    );
}
