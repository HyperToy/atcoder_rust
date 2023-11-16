use proconio::{marker::Usize1, *};
use std::collections::HashSet;

fn main() {
    input! {
        (n, m): (usize, usize),
        es: [(Usize1, Usize1); m],
        k: usize,
        xy: [(Usize1, Usize1); k],
        q: usize,
        queries: [(Usize1, Usize1); q],
    }
    let mut dsu = Dsu::new(n);
    for (u, v) in es {
        dsu.unite(u, v);
    }
    let mut restricted = HashSet::new();
    for (x, y) in xy {
        let mut rx = dsu.root(x);
        let mut ry = dsu.root(y);
        if rx > ry {
            (rx, ry) = (ry, rx);
        }
        restricted.insert((rx, ry));
    }
    for (p, q) in queries {
        let mut rp = dsu.root(p);
        let mut rq = dsu.root(q);
        if rp > rq {
            (rp, rq) = (rq, rp);
        }
        println!(
            "{}",
            if !restricted.contains(&(rp, rq)) {
                "Yes"
            } else {
                "No"
            }
        );
    }
}

struct Dsu {
    parent: Vec<usize>,
    size: Vec<i32>,
}
impl Dsu {
    fn new(n: usize) -> Self {
        let parent = (0..n).collect::<Vec<usize>>();
        let size = vec![1; n];
        Self { parent, size }
    }
    fn root(&self, x: usize) -> usize {
        let mut res = x;
        while self.parent[res] != res {
            res = self.parent[res];
        }
        res
    }
    fn unite(&mut self, u: usize, v: usize) {
        let root_u = self.root(u);
        let root_v = self.root(v);
        if root_u == root_v {
            return;
        }
        if self.size[root_u] < self.size[root_v] {
            self.parent[root_u] = root_v;
            self.size[root_v] += self.size[root_u];
        } else {
            self.parent[root_v] = root_u;
            self.size[root_u] += self.size[root_v];
        }
    }
    #[allow(dead_code)]
    fn same(&self, u: usize, v: usize) -> bool {
        self.root(u) == self.root(v)
    }
}
