use itertools::Itertools;
use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        n: usize, m: usize,
        s: Chars,
        c: [Usize1; n],
    }
    let mut v = vec![Vec::new(); m];
    for i in 0..n {
        v[c[i]].push(i);
    }
    let mut answer = s.clone();
    for j in 0..m {
        answer[v[j][0]] = s[*v[j].last().unwrap()];
        for i in 1..v[j].len() {
            answer[v[j][i]] = s[v[j][i - 1]];
        }
    }
    println!("{}", answer.iter().join(""));
}
