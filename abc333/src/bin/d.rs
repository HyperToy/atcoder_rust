use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        es: [(Usize1, Usize1); n - 1],
    }
    let mut g = vec![Vec::new(); n];
    for (u, v) in es {
        g[u].push(v);
        g[v].push(u);
    }
    let root = 0;
    println!(
        "{}",
        n - g[root].iter().map(|i| dfs(*i, root, &g)).max().unwrap()
    );
}
fn dfs(u: usize, pre: usize, g: &Vec<Vec<usize>>) -> usize {
    let mut size = 1;
    for v in &g[u] {
        if *v == pre {
            continue;
        }
        size += dfs(*v, u, g);
    }
    size
}
