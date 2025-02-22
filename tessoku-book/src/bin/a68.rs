use proconio::{marker::Usize1, *};

fn main() {
    input! {
        n: usize, m: usize,
        es: [(Usize1, Usize1, i32); m],
    }
    let mut maximum_flow = MaximumFlow::new(n);
    for (u, v, cap) in es {
        maximum_flow.add_edge(u, v, cap);
    }
    println!("{}", maximum_flow.max_flow(0, n - 1));
}

#[derive(Clone)]
struct Edge {
    to: usize,
    cap: i32,
    rev: usize,
}

struct MaximumFlow {
    size: usize,
    g: Vec<Vec<Edge>>,
}
impl MaximumFlow {
    fn new(size: usize) -> Self {
        let g = vec![Vec::new(); size];
        Self { size, g }
    }
    fn add_edge(&mut self, u: usize, v: usize, cap: i32) {
        let current_u = self.g[u].len();
        let current_v = self.g[v].len();
        self.g[u].push(Edge {
            to: v,
            cap,
            rev: current_v,
        });
        self.g[v].push(Edge {
            to: u,
            cap: 0,
            rev: current_u,
        });
    }
    fn dfs(&mut self, pos: usize, target: usize, f: i32, used: &mut Vec<bool>) -> i32 {
        if pos == target {
            return f;
        }
        used[pos] = true;
        for i in 0..self.g[pos].len() {
            let e = self.g[pos][i].clone();
            if e.cap == 0 {
                continue;
            }
            if used[e.to] {
                continue;
            }
            let flow = self.dfs(e.to, target, f.min(e.cap), used);
            if flow > 0 {
                self.g[pos][i].cap -= flow;
                self.g[e.to][e.rev].cap += flow;
                return flow;
            }
        }
        0
    }
    fn max_flow(&mut self, source: usize, target: usize) -> i32 {
        let mut total_flow = 0;
        loop {
            let mut used = vec![false; self.size];
            let f = self.dfs(source, target, std::i32::MAX, &mut used);
            if f == 0 {
                break;
            }
            total_flow += f;
        }
        total_flow
    }
}
