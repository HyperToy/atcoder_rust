use itertools::Itertools;
use proconio::input;

// todo: refactor
fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    let mut g = vec![Vec::new(); n];
    let mut start = 0;
    for i in 0..n {
        // a[i] -1 -> i の辺を張る
        if a[i] > 0 {
            g[a[i] as usize - 1].push(i);
        } else {
            start = i;
        }
    }
    let mut answer = Vec::new();
    dfs(start, &mut answer, &g);
    println!("{}", answer.iter().map(|a| a + 1).join(" "));
}
fn dfs(p: usize, v: &mut Vec<usize>, g: &Vec<Vec<usize>>) {
    v.push(p);
    for u in &g[p] {
        dfs(*u, v, g);
    }
}
