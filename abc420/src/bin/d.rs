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
    let dist = dijkstra(h * w * 2, graph, s);
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
    let mut res = vec![];
    if pos < h * w {
        let (i, j) = (pos / w, pos % w);
        if i > 0 {
            res.push(pos - w);
        }
        if i < h - 1 {
            res.push(pos + w);
        }
        if j > 0 {
            res.push(pos - 1);
        }
        if j < w - 1 {
            res.push(pos + 1);
        }
    } else {
        let ipos = switch(pos, (h, w));
        let (i, j) = (ipos / w, ipos % w);
        if i > 0 {
            res.push(switch(ipos - w, (h, w)));
        }
        if i < h - 1 {
            res.push(switch(ipos + w, (h, w)));
        }
        if j > 0 {
            res.push(switch(ipos - 1, (h, w)));
        }
        if j < w - 1 {
            res.push(switch(ipos + 1, (h, w)));
        }
    }
    res
}

fn switch(pos: usize, (h, w): (usize, usize)) -> usize {
    if pos < h * w {
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

fn dijkstra(n: usize, graph: Vec<Vec<(usize, i64)>>, start: usize) -> Vec<Option<i64>> {
    let mut dist = vec![None; n * 2];
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
