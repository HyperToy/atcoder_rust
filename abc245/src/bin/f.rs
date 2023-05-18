use proconio::*;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize, m: usize,
        vs: [(usize, usize); m],
    }
    let mut g = vec![Vec::new(); n];
    let mut rg = vec![Vec::new(); n];
    for mut v in vs {
        v.0 -= 1;
        v.1 -= 1;
        g[v.0].push(v.1);
        rg[v.1].push(v.0);
    }

    // 帰りがけじゅんの並び
    let mut vs = Vec::new();
    // すでに調べたか
    let mut used = vec![false; n];
    // 属する強連結成分のトポロジカル順序
    let mut cmp = vec![0; n];

    for v in 0..n {
        if !used[v] {
            dfs(v, &mut used, &g, &mut vs);
        }
    }
    let mut used = vec![false; n];
    let mut k = 0;
    for i in (0..vs.len()).rev() {
        if !used[vs[i]] {
            rdfs(vs[i], k, &mut used, &mut cmp, &rg);
            k += 1;
        }
    }

    let mut scs = vec![Vec::new(); n];
    for v in 0..n {
        scs[cmp[v]].push(v);
    }
    let mut ans = 0;
    let mut used = vec![false; n];
    let mut q = VecDeque::new();
    for i in 0..n {
        if scs[i].len() > 1 {
            // 各頂点をキューにツッコミ、そこから行ける頂点を BFS
            // ans += scs[i].len();
            for v in scs[i].iter().copied() {
                if !used[v] {
                    used[v] = true;
                    ans += 1;
                    q.push_back(v);
                }
            }
            while !q.is_empty() {
                let u = q.pop_front().unwrap();
                for v in rg[u].iter().copied() {
                    if !used[v] {
                        used[v] = true;
                        ans += 1;
                        q.push_back(v);
                    }
                }
            }
        }
    }
    println!("{}", ans);
}

fn dfs(v: usize, used: &mut Vec<bool>, g: &Vec<Vec<usize>>, vs: &mut Vec<usize>) -> () {
    used[v] = true;
    for i in 0..g[v].len() {
        if !used[g[v][i]] {
            dfs(g[v][i], used, &g, vs);
        }
    }
    vs.push(v)
}

fn rdfs(
    v: usize,
    k: usize,
    used: &mut Vec<bool>,
    cmp: &mut Vec<usize>,
    rg: &Vec<Vec<usize>>,
) -> () {
    used[v] = true;
    cmp[v] = k;
    for i in 0..rg[v].len() {
        if !used[rg[v][i]] {
            rdfs(rg[v][i], k, used, cmp, &rg);
        }
    }
}
