use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize, q: usize,
        es: [(Usize1, Usize1); n - 1],
        queries: [(Usize1, i32); q],
    }
    let mut g = vec![Vec::new(); n];
    for (a, b) in es {
        g[a].push(b);
        g[b].push(a);
    }
    let mut values = vec![0; n];
    for (p, x) in queries {
        values[p] += x;
    }
    let mut dfs = Dfs::new(g, values);
    dfs.exec(0, 0);
    println!("{}", dfs.values.iter().join(" "));
}

struct Dfs {
    g: Vec<Vec<usize>>,
    seen: Vec<bool>,
    values: Vec<i32>,
}
impl Dfs {
    fn new(g: Vec<Vec<usize>>, values: Vec<i32>) -> Self {
        let seen = vec![false; g.len()];
        Self { g, seen, values }
    }
    fn exec(&mut self, u: usize, pre: usize) {
        if self.seen[u] {
            return;
        }
        self.seen[u] = true;
        for v in self.g[u].clone() {
            if v == pre {
                continue;
            }
            self.values[v] += self.values[u];
            self.exec(v, u);
        }
    }
}
