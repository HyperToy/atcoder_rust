use itertools::Itertools;
use proconio::{marker::Usize1, *};

fn main() {
    input! {
        n: usize, t: Usize1,
        es: [(Usize1, Usize1); n - 1],
    }
    let mut g = vec![Vec::new(); n];
    for (a, b) in es {
        g[a].push(b);
        g[b].push(a);
    }
    let mut seen = vec![false; n];
    let mut answer = vec![0; n];
    dfs(t, &g, &mut seen, &mut answer);
    println!("{}", answer.iter().join(" "));
}

fn dfs(v: usize, g: &Vec<Vec<usize>>, seen: &mut Vec<bool>, answer: &mut Vec<i32>) -> i32 {
    if seen[v] {
        return -1;
    }
    seen[v] = true;
    let mut rank = -1;
    for u in g[v].clone() {
        rank = rank.max(dfs(u, &g, seen, answer));
    }
    answer[v] = rank + 1;
    answer[v]
}
