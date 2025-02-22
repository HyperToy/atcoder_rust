use itertools::Itertools;
use proconio::{marker::Usize1, *};

fn main() {
    input! {
        n: usize, m: usize,
        es: [(Usize1, Usize1); m],
    }
    let mut g = vec![Vec::new(); n];
    for (u, v) in es {
        g[u].push(v);
        g[v].push(u);
    }
    let mut seen = vec![false; n];
    let mut answer: Vec<usize> = Vec::new();
    dfs(0, 0, n - 1, &g, &mut seen, &mut answer);
    println!("{}", answer.iter().map(|x| x + 1).join(" "));
}

fn dfs(
    pos: usize,
    pre: usize,
    target: usize,
    g: &Vec<Vec<usize>>,
    seen: &mut Vec<bool>,
    answer: &mut Vec<usize>,
) -> bool {
    if seen[pos] {
        return false;
    }
    seen[pos] = true;
    answer.push(pos);
    if pos == target {
        return true;
    }
    for v in g[pos].clone() {
        if v == pre {
            continue;
        }
        if dfs(v, pos, target, &g, seen, answer) {
            return true;
        }
    }
    answer.pop();
    false
}
