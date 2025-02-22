use ac_library::ModInt998244353 as Mint;
use itertools::Itertools;
use proconio::{input, marker::Chars};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize, k: usize,
        s: Chars,
    }
    let ngs = k_palindromes(k);
    let l = 1 << k;
    let s = s
        .iter()
        .map(|&c| match c {
            'A' => vec![0],
            'B' => vec![1],
            '?' => vec![0, 1],
            _ => unreachable!(),
        })
        .collect_vec();
    let mut dp = HashMap::new();
    dp.insert(0, Mint::from(1));
    for i in 0..n {
        let mut next_dp = HashMap::new();
        for (&j, &count) in &dp {
            for &c in &s[i] {
                let nj = (j * 2 + c) % l;
                *(next_dp.entry(nj).or_insert(Mint::from(0))) += count;
            }
        }
        dp = next_dp
            .into_iter()
            .filter(|&(j, _)| i < k - 1 || !ngs.contains(&j))
            .collect::<HashMap<_, _>>();
    }
    println!("{}", dp.iter().map(|(_, v)| v).sum::<Mint>());
}

fn k_palindromes(k: usize) -> HashSet<usize> {
    (0..(1 << k))
        .map(|mut v| {
            let mut res = vec![];
            while v > 0 || res.len() < k {
                res.push(v % 2);
                v /= 2;
            }
            res
        })
        .filter(|vs| (0..k).all(|i| vs[i] == vs[k - 1 - i]))
        .map(|vs| vs.iter().enumerate().map(|(i, &v)| v << i).sum())
        .collect::<HashSet<_>>()
}
