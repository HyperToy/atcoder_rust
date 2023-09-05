use proconio::{marker::Usize1, *};

fn main() {
    input! {
        n: usize, m: usize,
        mut es: [(Usize1, Usize1, i64); m],
    }
    es.sort_by(|a, b| b.2.cmp(&a.2));
    let mut dsu = Dsu::new(n);
    let mut answer = 0;
    for (u, v, w) in es {
        if dsu.same(u, v) {
            continue;
        }
        dsu.unite(u, v);
        answer += w;
    }
    println!("{}", answer);
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
    fn same(&self, u: usize, v: usize) -> bool {
        self.root(u) == self.root(v)
    }
}
