use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize, m: usize,
        es: [(Usize1, Usize1); m],
    }
    let mut dsu = Dsu::new(n);
    for (a, b) in es {
        dsu.unite(a, b);
    }
    let mut answer = -(m as i64);
    for i in 0..n {
        if dsu.is_root(i) {
            let s = dsu.size[i];
            answer += s * (s - 1) / 2;
        }
    }
    println!("{}", answer);
}

struct Dsu {
    parent: Vec<usize>,
    size: Vec<i64>,
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
    fn is_root(&self, u: usize) -> bool {
        self.parent[u] == u
    }
    fn unite(&mut self, u: usize, v: usize) {
        let root_u = self.root(u);
        let root_v = self.root(v);
        if root_u == root_v {
            return;
        }
        let (root_u, root_v) = if self.size[root_u] < self.size[root_v] {
            (root_u, root_v)
        } else {
            (root_v, root_u)
        };
        self.parent[root_u] = root_v;
        self.size[root_v] += self.size[root_u];
    }
    #[allow(dead_code)]
    fn same(&self, u: usize, v: usize) -> bool {
        self.root(u) == self.root(v)
    }
}
