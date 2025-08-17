use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        t: usize,
        qs: [(usize, Chars); t],
    }
    println!("{}", qs.into_iter().map(|(_, q)| solve(q)).join("\n"));
}

fn solve(q: Vec<char>) -> i32 {
    q.iter()
        .fold([Some(0), None, None], |dp, &c| {
            [
                dp[0].map(|v| v + cost(c == '0')),
                min_option(dp[0], dp[1]).map(|v| v + cost(c == '1')),
                min_option(dp[1], dp[2]).map(|v| v + cost(c == '0')),
            ]
        })
        .iter()
        .filter_map(|&x| x)
        .min()
        .unwrap()
}

fn min_option(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    match (a, b) {
        (Some(x), Some(y)) => Some(x.min(y)),
        _ => a.or(b),
    }
}

fn cost(b: bool) -> i32 {
    if b {
        0
    } else {
        1
    }
}
