use proconio::{input, marker::Chars};
use std::{cmp::Reverse, collections::BinaryHeap};

fn main() {
    input! {
        h: usize, w: usize,
        a: [Chars; h],
    }
    let a = a.into_iter().flatten().collect::<Vec<_>>();
    let s = a.iter().position(|&c| c == 'S').unwrap();
    let g = a.iter().position(|&c| c == 'G').unwrap();

    let b = a
        .iter()
        .map(|&c| match c {
            'o' => 'x',
            'x' => 'o',
            _ => c,
        })
        .collect::<Vec<_>>();
    let a = a.into_iter().chain(b.into_iter()).collect::<Vec<_>>();
    let mut es = vec![];
    for (i, c) in a.iter().enumerate() {
        if *c == '#' || *c == 'x' {
            continue;
        }
        for j in neighbors(i, (h, w)) {
            let d = a[j];
            match d {
                '.' | 'S' | 'G' | 'o' => es.push((i, j, 1)),
                '?' => es.push((i, switch(j, (h, w)), 1)),
                '#' | 'x' => (),
                _ => unreachable!(),
            }
        }
    }

    let graph = make_graph(h * w * 2, es);
    let dist = dijkstra(graph, s);
    println!(
        "{}",
        match (dist[g], dist[switch(g, (h, w))]) {
            (None, None) => -1,
            (None, Some(e)) => e,
            (Some(d), None) => d,
            (Some(d), Some(e)) => d.min(e),
        }
    );
}

fn neighbors(pos: usize, (h, w): (usize, usize)) -> Vec<usize> {
    if is_front(pos, (h, w)) {
        neighbors_front(pos, (h, w))
    } else {
        neighbors_front(switch(pos, (h, w)), (h, w))
            .iter()
            .map(|&x| switch(x, (h, w)))
            .collect()
    }
}

fn neighbors_front(pos: usize, (h, w): (usize, usize)) -> Vec<usize> {
    let (i, j) = decode(pos, (h, w));
    [
        (if i > 0 { Some(i - 1) } else { None }, Some(j)), // 上
        (if i < h - 1 { Some(i + 1) } else { None }, Some(j)), // 下
        (Some(i), if j > 0 { Some(j - 1) } else { None }), // 左
        (Some(i), if j < w - 1 { Some(j + 1) } else { None }), // 右
    ]
    .iter()
    .filter_map(|(ni, nj)| match (ni, nj) {
        (Some(ni), Some(nj)) => Some((*ni, *nj)),
        _ => None,
    })
    .map(|(ni, nj)| encode((ni, nj), (h, w)))
    .collect()
}

fn encode((i, j): (usize, usize), (_h, w): (usize, usize)) -> usize {
    i * w + j
}

fn decode(pos: usize, (_h, w): (usize, usize)) -> (usize, usize) {
    (pos / w, pos % w)
}

fn is_front(pos: usize, (h, w): (usize, usize)) -> bool {
    pos < h * w
}

fn switch(pos: usize, (h, w): (usize, usize)) -> usize {
    if is_front(pos, (h, w)) {
        pos + h * w
    } else {
        pos - h * w
    }
}

fn make_graph(n: usize, es: Vec<(usize, usize, i64)>) -> Vec<Vec<(usize, i64)>> {
    let mut graph = vec![vec![]; n];
    for &(u, v, w) in &es {
        graph[u].push((v, w));
    }
    graph
}

fn dijkstra(graph: Vec<Vec<(usize, i64)>>, start: usize) -> Vec<Option<i64>> {
    let n = graph.len();
    let mut dist = vec![None; n];
    let mut queue = BinaryHeap::new();
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
