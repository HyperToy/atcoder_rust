use itertools::Itertools;
use proconio::{marker::Usize1, *};

fn main() {
    input! {
        n: usize, m: usize,
        a: [Usize1; m],
    }
    let mut answer = vec![m; n];
    for i in 0..m {
        answer[a[i]] -= 1;
    }
    println!("{}", answer.iter().join(" "));
}
