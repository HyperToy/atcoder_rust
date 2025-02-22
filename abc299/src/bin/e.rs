use itertools::Itertools;
use proconio::{input, marker::Usize1};
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize, m: usize,
        es: [(Usize1, Usize1); m],
        k: usize,
        pd: [(Usize1, i32); k],
    }
    let g = make_graph(n, es);
    let mut dist = vec![vec![std::i32::MAX; n]; n];
    let mut queue = VecDeque::new();
    for i in 0..n {
        dist[i][i] = 0;
        queue.push_back(i);
        while !queue.is_empty() {
            let u = queue.pop_front().unwrap();
            for v in &g[u] {
                if dist[i][*v] <= dist[i][u] + 1 {
                    continue;
                }
                dist[i][*v] = dist[i][u] + 1;
                queue.push_back(*v);
            }
        }
    }

    let mut color = vec![Color::UnCertain; n];
    for (p, d) in &pd {
        for q in 0..n {
            if dist[*p][q] < *d {
                color[q] = Color::White;
            }
        }
    }
    for i in 0..n {
        if color[i] == Color::UnCertain {
            color[i] = Color::Black;
        }
    }

    let mut ok = true;
    for (p, d) in pd {
        let mut find = false;
        for i in 0..n {
            if dist[p][i] == d && color[i] == Color::Black {
                find = true;
            }
        }
        if !find {
            ok = false;
        }
    }
    println!(
        "{}",
        if ok {
            "Yes ".to_string()
                + color
                    .iter()
                    .map(|c| match c {
                        Color::Black => "1",
                        Color::White => "0",
                        Color::UnCertain => ".",
                    })
                    .join("")
                    .as_str()
        } else {
            "No".to_string()
        }
    );
}

fn make_graph(n: usize, es: Vec<(usize, usize)>) -> Vec<Vec<usize>> {
    let mut g = vec![Vec::new(); n];
    for (u, v) in es {
        g[u].push(v);
        g[v].push(u);
    }
    g
}

#[derive(Clone, PartialEq, Eq)]
enum Color {
    Black,
    White,
    UnCertain,
}
