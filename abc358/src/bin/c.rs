use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize, m: u32,
        s: [Chars; n],
    }
    let s = s
        .iter()
        .map(|cs| {
            cs.iter()
                .map(|&c| match c {
                    'o' => 1,
                    'x' => 0,
                    _ => unreachable!(),
                })
                .enumerate()
                .map(|(i, v)| v << i)
                .sum::<usize>()
        })
        .collect_vec();
    println!(
        "{}",
        (0..1usize << n)
            .filter(|&msk| (0..n)
                .fold(0, |any, i| any | if (msk >> i) & 1 == 1 { s[i] } else { 0 })
                .count_ones()
                == m)
            .map(|msk| msk.count_ones())
            .min()
            .unwrap()
    )
}
