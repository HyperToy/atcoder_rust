use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize, q: usize,
        mut a: [i64; n], mut b: [i64; n],
        qs: [(char, Usize1, i64); q],
    }
    let mut answer = vec![];
    let mut min_sum = a.iter().zip(b.iter()).map(|(a, b)| a.min(b)).sum::<i64>();
    for (c, x, v) in qs {
        let (pa, pb) = (a[x], b[x]);
        match c {
            'A' => a[x] = v,
            'B' => b[x] = v,
            _ => unreachable!(),
        };
        let (na, nb) = (a[x], b[x]);
        let diff = na.min(nb) - pa.min(pb);
        min_sum = min_sum + diff;
        answer.push(min_sum);
    }
    println!("{}", answer.iter().join("\n"));
}
