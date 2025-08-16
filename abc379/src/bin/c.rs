use itertools::Itertools;
use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize, m: usize,
        x: [usize; m],
        a: [usize; m],
    }
    let xa = x.into_iter().zip(a).collect::<BTreeSet<_>>();
    println!(
        "{}",
        solve(n, xa)
            .map(|x| x.to_string())
            .unwrap_or("-1".to_string())
    );
}

fn solve(n: usize, xa: BTreeSet<(usize, usize)>) -> Option<usize> {
    if xa.iter().map(|(_, a)| a).sum::<usize>() != n {
        return None;
    }
    let culm = xa
        .iter()
        .scan(0, |acc, (x, a)| {
            *acc += a;
            Some((*x, *acc))
        })
        .chain(std::iter::once((0, 0)))
        .chain(std::iter::once((n + 1, n + 1)))
        .collect::<BTreeSet<_>>();
    if culm
        .iter()
        .tuple_windows()
        .any(|((_, x), (j, _))| *j > *x + 1)
    {
        return None;
    }
    let res = culm
        .iter()
        .tuple_windows()
        .map(|((i, x), (j, _))| {
            arithmetic_sum(x - i) - if x > j { arithmetic_sum(x - j) } else { 0 }
        })
        .sum::<usize>();
    Some(res)
}

fn arithmetic_sum(x: usize) -> usize {
    x * (x + 1) / 2
}
