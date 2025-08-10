use proconio::{input, marker::Usize1};
use std::{cmp::Reverse, collections::BinaryHeap};

fn main() {
    input! {
        n: usize, m: usize, x: i64,
        es: [(Usize1, Usize1); m],
    }
    let graph = make_graph(n, es, x);
    let dist = dijkstra(n, graph);
    println!("{}", dist[n - 1].min(dist[n * 2 - 1]).unwrap());
}

fn make_graph(n: usize, es: Vec<(usize, usize)>, x: i64) -> Vec<Vec<(usize, i64)>> {
    // n 〜 2n-1 はパラレルワールド
    let mut graph = vec![vec![]; n * 2];
    for &(u, v) in &es {
        graph[u].push((v, 1));
        // パラレルワールドで逆向きに辺を張る
        graph[v + n].push((u + n, 1));
    }
    // パラレルワールドと行き来する辺を張る
    for i in 0..n {
        graph[i].push((i + n, x));
        graph[i + n].push((i, x));
    }
    graph
}

fn dijkstra(n: usize, graph: Vec<Vec<(usize, i64)>>) -> Vec<Option<i64>> {
    let mut dist = vec![None; n * 2];
    let mut queue = BinaryHeap::new();
    let start = 0;
    dist[start] = Some(0);
    queue.push((Reverse(dist[start]), start));
    while let Some((Reverse(Some(d)), u)) = queue.pop() {
        for &(v, w) in &graph[u] {
            if dist[v].is_none() || dist[v].is_some_and(|nd| nd > d + w) {
                dist[v] = Some(d + w);
                queue.push((Reverse(dist[v]), v));
            }
        }
    }
    dist
}
