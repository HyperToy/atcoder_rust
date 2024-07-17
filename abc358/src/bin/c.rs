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
            .filter(|&msk| {
                let mut any = 0;
                for i in 0..n {
                    if (msk >> i) & 1 == 1 {
                        any |= s[i];
                    }
                }
                any.count_ones() == m
            })
            .map(|msk| msk.count_ones())
            .min()
            .unwrap()
    )
}
