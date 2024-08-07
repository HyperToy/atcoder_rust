use itertools::Itertools;
use proconio::{input, marker::Chars};

type Hand = usize;
const R: Hand = 0;
const P: Hand = 1;
const S: Hand = 2;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let s = s
        .iter()
        .map(|&c| match c {
            'R' => R,
            'P' => P,
            'S' => S,
            _ => unreachable!(),
        })
        .collect_vec();
    let mut dp = vec![[std::i32::MIN; 3]; n];
    dp[0][win(s[0])] = 1;
    dp[0][s[0]] = 0;
    for i in 1..n {
        for j in 0..3 {
            if s[i] == win(j) {
                continue;
            }
            for k in 0..3 {
                if k == j {
                    continue;
                }
                dp[i][j] = dp[i][j].max(dp[i - 1][k] + if j == win(s[i]) { 1 } else { 0 });
            }
        }
    }
    println!("{}", dp[n - 1].iter().max().unwrap());
}
fn win(h: Hand) -> Hand {
    match h {
        R => P,
        P => S,
        S => R,
        _ => unreachable!(),
    }
}
