use itertools::Itertools;
use proconio::*;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize, d: f64,
        xy: [(f64, f64); n],
    }
    let mut g = vec![Vec::new(); n];
    for i in 0..n {
        for j in 0..i {
            if distance(xy[i], xy[j]) <= d {
                g[i].push(j);
                g[j].push(i);
            }
        }
    }
    let mut queue = VecDeque::new();
    queue.push_back(0);
    let mut infected = vec![false; n];
    infected[0] = true;
    while !queue.is_empty() {
        let u = queue.pop_front().unwrap();
        for v in g[u].clone() {
            if infected[v] {
                continue;
            }
            queue.push_back(v);
            infected[v] = true;
        }
    }
    println!(
        "{}",
        infected
            .into_iter()
            .map(|x| if x { "Yes" } else { "No" })
            .join(" ")
    );
}
fn distance(a: (f64, f64), b: (f64, f64)) -> f64 {
    ((a.0 - b.0).powf(2.) + (a.1 - b.1).powf(2.)).sqrt()
}
