use std::vec;

use proconio::{input, marker::Usize1};

fn add_edge(g: &mut Vec<Vec<usize>>, rg: &mut Vec<Vec<usize>>, from: usize, to: usize) {
    g[from].push(to);
    rg[to].push(from);
}

fn dfs(v: usize, g: &Vec<Vec<usize>>, seen: &mut Vec<bool>, vs: &mut Vec<usize>) {
    seen[v] = true;
    for &u in &g[v] {
        if !seen[u] {
            dfs(u, g, seen, vs);
        }
    }
    vs.push(v); // 帰りがけ順に番号をつける
}

fn rdfs(v: usize, rg: &Vec<Vec<usize>>, seen: &mut Vec<bool>, cmp: &mut Vec<usize>, k: usize) {
    seen[v] = true;
    cmp[v] = k;
    for &u in &rg[v] {
        if !seen[u] {
            rdfs(u, rg, seen, cmp, k);
        }
    }
}

// 強連結成分の個数を返す。
fn scc(n: usize, g: &Vec<Vec<usize>>, rg: &Vec<Vec<usize>>) -> (usize, Vec<usize>) {
    let mut seen = vec![false; n];
    let mut vs: Vec<usize> = Vec::new();

    for v in 0..n {
        if !seen[v] {
            dfs(v, g, &mut seen, &mut vs);
        }
    }

    seen.fill(false);
    let mut cmp = vec![0; n];
    let mut k = 0;
    for &v in vs.iter().rev() {
        if !seen[v] {
            rdfs(v, rg, &mut seen, &mut cmp, k);
            k += 1;
        }
    }
    (k, cmp)
}

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    }
    let mut g = vec![Vec::new(); n];
    let mut rg = vec![Vec::new(); n];

    for (from, to) in a.into_iter().enumerate() {
        add_edge(&mut g, &mut rg, from, to);
    }

    let (k, cmp) = scc(n, &g, &rg);

    // 強連結成分を潰して 1頂点にしたグラフ
    let mut g = vec![Vec::new(); k];
    let mut rg2 = vec![Vec::new(); k];
    for i in 0..n {
        for &v in &rg[i] {
            if cmp[i] != cmp[v] {
                g[cmp[i]].push(cmp[v]);
                rg2[cmp[v]].push(cmp[i]);
            }
        }
    }

    let mut answer = 0;

    let mut size = vec![0; k]; // 各強連結成分のサイズ
    for i in 0..n {
        size[cmp[i]] += 1;
    }
    for &x in &size {
        answer += x * (x - 1);
    }

    // 入次数が 0 の頂点を探す
    let mut roots = Vec::new();
    for u in 0..k {
        if rg2[u].len() == 0 {
            roots.push(u);
        }
    }
    for u in roots {
        dfs2(&mut answer, &mut g, &size, u);
    }

    // 自分自身へは行ける
    answer += n;

    println!("{}", answer);
}

fn dfs2(answer: &mut usize, g: &Vec<Vec<usize>>, size: &Vec<usize>, pos: usize) -> usize {
    let mut res = 1;
    for &v in &g[pos] {
        let now = dfs2(answer, g, size, v);
        *answer += size[pos] * now;
        res += now;
    }
    res
}
