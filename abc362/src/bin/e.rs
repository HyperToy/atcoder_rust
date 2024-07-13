use ac_library::ModInt998244353 as Mint;
use itertools::Itertools;
use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut answer = vec![Mint::from(0); n];
    answer[0] = Mint::from(n);
    if n >= 2 {
        answer[1] = Mint::from(n * (n - 1) / 2);
    }
    let mut set = HashSet::new();
    for i in 0..n {
        for j in i + 1..n {
            if a[j] == a[i] {
                continue;
            }
            set.insert(vec![i, j]);
        }
    }
    for k in 2..n {
        let mut n_set = HashSet::new();
        for s in set {
            let d = a[s[1]] - a[s[0]];
            let tail = *s.last().unwrap();
            for i in tail + 1..n {
                if a[tail] + d == a[i] {
                    let mut s = s.clone();
                    s.push(i);
                    n_set.insert(s);
                }
            }
        }
        answer[k] = Mint::from(n_set.len());
        set = n_set;
    }
    let count = a
        .into_iter()
        .counts()
        .into_iter()
        .map(|(k, v)| (k, Mint::from(v)))
        .collect_vec();
    let mut counts = vec![count];
    for i in 1..n {
        let n_counts = counts[i - 1]
            .iter()
            .zip(counts[0].iter())
            .map(|(&(k, v), &(_, n))| (k, v * (n - i) / (i + 1)))
            .collect_vec();
        counts.push(n_counts);
    }
    for k in 2..n {
        answer[k] += counts[k].iter().map(|&(_, v)| v).sum::<Mint>();
    }
    println!("{}", answer.into_iter().join(" "));
}
