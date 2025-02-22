use itertools::Itertools;
use proconio::{marker::Usize1, *};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n - 1],
    }
    let mut answer = vec![0; n];
    for i in (1..n).rev() {
        answer[a[i - 1]] += answer[i] + 1;
    }
    println!("{}", answer.iter().join(" "));
}
