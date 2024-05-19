use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize, m: usize,
        mut a: [Usize1; m],
    }
    if a.contains(&0) || a.contains(&(n - 1)) {
        println!("-1");
        return;
    }
    a.sort();
    let mut answer = (0..n).collect::<Vec<_>>();
    for &a in &a {
        let i = answer[a];
        let j = answer[a + 1];
        answer[a] = j;
        answer[a + 1] = i;
    }
    println!("{}", answer.into_iter().map(|x| x + 1).join(" "));
}
