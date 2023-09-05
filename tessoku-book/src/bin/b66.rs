use itertools::Itertools;
use proconio::{marker::Usize1, *};

fn main() {
    input! {
        n: usize, m: usize,
        es: [(Usize1, Usize1); m],
        q: usize,
    }
    let mut connected = vec![true; m];
    let mut qs = Vec::new();
    for _ in 0..q {
        input! {
            t: usize,
        }
        match t {
            1 => {
                input! {
                    x: Usize1
                }
                connected[x] = false;
                qs.push((t, x, 0));
            }
            2 => {
                input! {
                    u: Usize1, v: Usize1,
                }
                qs.push((t, u, v));
            }
            _ => unreachable!(),
        }
    }

    let mut dsu = Dsu::new(n);
    for i in 0..m {
        if connected[i] {
            let (u, v) = es[i];
            dsu.unite(u, v);
        }
    }
    let mut answer = Vec::new();
    for (t, u, v) in qs.into_iter().rev() {
        match t {
            1 => {
                let (u, v) = es[u];
                dsu.unite(u, v);
            }
            2 => {
                answer.push(dsu.same(u, v));
            }
            _ => unreachable!(),
        }
    }
    println!(
        "{}",
        answer
            .into_iter()
            .rev()
            .map(|x| if x { "Yes" } else { "No" })
            .join(" ")
    );
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
