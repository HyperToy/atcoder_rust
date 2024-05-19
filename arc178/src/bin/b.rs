use ac_library::ModInt998244353 as Mint;
use itertools::Itertools;
use proconio::input;

// バグ取り、間に合わず
fn main() {
    input! {
        t: usize,
        queries: [(u64, u64, u64); t],
    }
    let mut answer = Vec::new();
    for (a, b, c) in queries {
        let (a, b) = if a <= b { (a, b) } else { (b, a) };
        if c < b || b + 1 < c {
            answer.push(Mint::from(0));
            continue;
        }
        answer.push(calc(a, b, c));
    }
    println!("{}", answer.into_iter().join("\n"));
}

fn calc(a: u64, b: u64, c: u64) -> Mint {
    let aa = Mint::from(10).pow(a - 1) * 9;
    let bb = Mint::from(10).pow(b - 1) * 9;
    if a == 1 {
        if b == 1 {
            if c == 1 {
                Mint::from(36)
            } else {
                assert!(c == 2);
                Mint::from(45)
            }
        } else if b + 1 == c {
            Mint::from(45)
        } else {
            bb * (aa - 1) - calc(a, b, c - 1)
        }
    } else {
        if b == c {
            if a == b {
                let x = bb - Mint::from(10).pow(a - 1);
                x * (x + 1) / 2
            } else {
                aa * (aa + 1) / 2 + (bb - Mint::from(10).pow(a)) * aa
            }
        } else {
            aa * bb - calc(a, b, c - 1)
        }
    }
}
