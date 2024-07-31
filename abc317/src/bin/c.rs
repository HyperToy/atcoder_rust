use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize, m: usize,
        es: [((Usize1, Usize1), i64); m],
    }
    let mut g = vec![vec![]; n];
    for ((u, v), w) in es {
        g[u].push((v, w));
        g[v].push((u, w));
    }
    println!("{}", (0..n).map(|i| rec(i, 1 << i, &g)).max().unwrap());
}
fn rec(u: usize, already: usize, g: &Vec<Vec<(usize, i64)>>) -> i64 {
    g[u].iter()
        .filter(|&&(v, _)| already >> v & 1 == 0)
        .map(|&(v, w)| rec(v, already | 1 << v, g) + w)
        .max()
        .unwrap_or(0)
}
