use itertools::Itertools;
use proconio::*;

fn main() {
    input! {
        n: usize, k: f64,
        a: [f64; n],
    }
    let mut ng = 1.;
    let mut ok = 1_000_000_000.;
    while ng + 0.000001 < ok {
        let wj = (ng + ok) / 2.;
        let mut count = 0.;
        for x in a.clone() {
            count += (x / wj).floor();
        }
        if count <= k {
            ok = wj;
        } else {
            ng = wj;
        }
    }
    let mut answer = vec![0.; n];
    for i in 0..n {
        answer[i] = (a[i] / ok).floor();
    }
    println!("{}", answer.iter().join(" "));
}

/*
#[derive(PartialEq, Eq)]
struct Rational {
    number: i64,
    demon: i64,
}
impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.number * other.demon).partial_cmp(&(other.number * self.demon))
    }
}
impl Ord for Rational {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.number * other.demon).cmp(&(other.number * self.demon))
    }
}
impl Rational {
    fn new(number: i64, demon: i64) -> Self {
        Self { number, demon }
    }
}
 */
