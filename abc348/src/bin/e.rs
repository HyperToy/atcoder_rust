use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        es: [(Usize1, Usize1); n - 1],
        c: [i64; n],
    }
    let g = graph(n, &es);
    let total = c.iter().sum::<i64>();

    let mut x = n; // 木の重心 (どの部分木の重みの和も全体の重みの和の半分以下であるような頂点)
    dfs1(0, n, &g, &c, total, &mut x);

    let answer = dfs2(x, 0, n, &g, &c);
    println!("{}", answer);
}

type Graph = Vec<Vec<usize>>;
fn graph(n: usize, es: &Vec<(usize, usize)>) -> Graph {
    let mut g = vec![Vec::new(); n];
    for (u, v) in es {
        g[*u].push(*v);
        g[*v].push(*u);
    }
    g
}

// 重心を見つける
fn dfs1(v: usize, p: usize, g: &Graph, c: &Vec<i64>, total: i64, x: &mut usize) -> i64 {
    let mut res = c[v]; // v以下の部分木の重みの和
    let mut max = 0; // vから出ている部分木の重みの和の最大値
    for &u in &g[v] {
        if u == p {
            continue;
        }
        let now = dfs1(u, v, g, c, total, x);
        max = max.max(now);
        res += now;
    }
    max = max.max(total - res); // 上方向に確認
    if max * 2 <= total {
        *x = v;
    }
    res
}

// 重心からの重み付き距離の和を求める
fn dfs2(v: usize, dist: i64, p: usize, g: &Graph, c: &Vec<i64>) -> i64 {
    let mut res = dist * c[v];
    for &u in &g[v] {
        if u == p {
            continue;
        }
        res += dfs2(u, dist + 1, v, g, c);
    }
    res
}
