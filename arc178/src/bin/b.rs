use ac_library::ModInt998244353 as Mint;
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        t: usize,
        queries: [(u64, u64, u64); t],
    }
    println!(
        "{}",
        queries
            .into_iter()
            .map(|(a, b, c)| calc(a, b, c))
            .join("\n")
    );
}

fn calc(a: u64, b: u64, c: u64) -> Mint {
    let (a, b) = if a <= b { (a, b) } else { (b, a) };
    if c < b || b + 1 < c {
        return Mint::from(0);
    }
    let aa = Mint::from(10).pow(a - 1) * 9;
    let bb = Mint::from(10).pow(b - 1) * 9;
    if b == c {
        if a == b {
            let x = bb - Mint::from(10).pow(a - 1);
            sum(x)
        } else {
            sum(aa) + (bb - Mint::from(10).pow(a)) * aa
        }
    } else {
        aa * bb - calc(a, b, c - 1)
    }
}

fn sum(x: Mint) -> Mint {
    x * (x + 1) / 2
}
