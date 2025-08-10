use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        x: [i64; n],
        es: [(Usize1, Usize1, i64); n - 1],
    }
    let graph = make_graph(n, es);
    let answer = dfs(&graph, &x, 0, None);
    println!("{}", answer.0);
}

fn make_graph(n: usize, es: Vec<(usize, usize, i64)>) -> Vec<Vec<(usize, i64)>> {
    let mut graph = vec![vec![]; n];
    for (u, v, w) in es {
        graph[u].push((v, w));
        graph[v].push((u, w));
    }
    graph
}

fn dfs(graph: &[Vec<(usize, i64)>], x: &[i64], v: usize, p: Option<usize>) -> (i64, i64) {
    let (mut cost, mut count) = (0, x[v]);
    for &(u, w) in &graph[v] {
        if Some(u) != p {
            let (c1, c2) = dfs(graph, x, u, Some(v));
            cost += c1 + c2.abs() * w;
            count += c2;
        }
    }
    (cost, count)
}
