use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        t: usize,
        ns: [u64; t],
    }
    println!(
        "{}",
        ns.into_iter()
            .map(|n| {
                let v = prime_factorize(n);
                v.len() > 1 && n >= v.iter().map(|(p, e)| p.pow(*e)).sum::<u64>()
            })
            .map(|b| if b { "Yes" } else { "No" })
            .join("\n")
    );
}

fn prime_factorize(mut n: u64) -> Vec<(u64, u32)> {
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
