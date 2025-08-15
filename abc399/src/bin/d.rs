use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            a: [Usize1; n * 2],
        }
        println!("{}", solve(n, a));
    }
}

fn solve(n: usize, a: Vec<usize>) -> usize {
    let mut pos = vec![vec![]; n];
    for i in 0..n * 2 {
        pos[a[i]].push(i);
    }
    a.iter()
        .tuple_windows()
        .map(|(a, b)| {
            let (ia, ja) = (pos[*a][0], pos[*a][1]);
            let (ib, jb) = (pos[*b][0], pos[*b][1]);
            ((ia, ja, a), (ib, jb, b))
        })
        .filter(|((ia, ja, _), (ib, jb, _))| {
            abs(*ia, *ib) == 1 && abs(*ja, *jb) == 1 && abs(*ia, *ja) > 1 && abs(*ib, *jb) > 1
        })
        .map(|((_, _, a), (_, _, b))| if a < b { (a, b) } else { (b, a) })
        .sorted()
        .dedup()
        .count()
}

fn abs(a: usize, b: usize) -> usize {
    a.max(b) - a.min(b)
}
