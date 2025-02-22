use proconio::{marker::Usize1, *};

fn main() {
    input! {
        n: usize, q: usize,
        qs: [(u8, Usize1, Usize1); q],
    }
    let mut dsu = Dsu::new(n);
    for (t, u, v) in qs {
        match t {
            1 => {
                dsu.unite(u, v);
            }
            2 => {
                println!("{}", if dsu.same(u, v) { "Yes" } else { "No" });
            }
            _ => unreachable!(),
        }
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
    fn same(&self, u: usize, v: usize) -> bool {
        self.root(u) == self.root(v)
    }
}
