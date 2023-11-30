use proconio::{input, marker::Usize1};

// refactor combination の iteration
fn main() {
    input! {
        n: usize, m: usize, k: i64,
        es: [(Usize1, Usize1, i64); m],
    }
    let es = es
        .into_iter()
        .map(|(u, v, w)| Edge::new(u, v, w))
        .collect::<Vec<_>>();
    let input = Input { n, m, k, es };

    let mut answer = 1_000_000_000_000_000_000;
    dfs(0, &mut Vec::new(), &mut answer, &input);
    println!("{}", answer);
}

struct Input {
    n: usize,
    m: usize,
    k: i64,
    es: Vec<Edge>,
}

fn dfs(i: usize, is: &mut Vec<usize>, answer: &mut i64, input: &Input) {
    if is.len() == input.n - 1 {
        let mut dsu = Dsu::new(input.n);
        let mut ok = true;
        let mut now = 0;
        for ei in is.clone() {
            let e = &input.es[ei];
            if dsu.same(e.u, e.v) {
                ok = false;
                break;
            }
            dsu.unite(e.u, e.v);
            now = (now + e.w) % input.k;
        }
        if ok && now < *answer {
            *answer = now;
        }
        return;
    }
    if i == input.m {
        return;
    }
    dfs(i + 1, is, answer, &input);
    is.push(i);
    dfs(i + 1, is, answer, &input);
    is.pop();
}

struct Edge {
    u: usize,
    v: usize,
    w: i64,
}
impl Edge {
    fn new(u: usize, v: usize, w: i64) -> Self {
        Self { u, v, w }
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
