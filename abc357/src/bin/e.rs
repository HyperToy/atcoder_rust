use itertools::Itertools;
use proconio::{input, marker::Usize1};

#[derive(Clone)]
#[allow(dead_code)]
struct Edge {
    from: usize,
    to: usize,
    val: i64,
}
impl Edge {
    fn new(from: usize, to: usize, val: i64) -> Self {
        Self { from, to, val }
    }
}
struct CycleDetection {
    g: Vec<Edge>,
    seen: Vec<bool>,
    finished: Vec<bool>,
    history: Vec<usize>,
}
impl CycleDetection {
    fn new(g: Vec<Edge>) -> Self {
        let l = g.len();
        let seen = vec![false; l];
        let finished = vec![false; l];
        let history = Vec::new();
        Self {
            g,
            seen,
            finished,
            history,
        }
    }

    // return the vertex where cycle is detected
    fn search(&mut self, mut v: usize) -> Option<usize> {
        loop {
            self.seen[v] = true;
            self.history.push(v);
            v = self.g[v].to;

            if self.finished[v] {
                v = std::usize::MAX;
                break;
            }
            if self.seen[v] {
                break;
            }
        }
        self.pop_history();
        if v == std::usize::MAX {
            None
        } else {
            Some(v)
        }
    }

    // pop history
    fn pop_history(&mut self) {
        while let Some(v) = self.history.pop() {
            self.finished[v] = true;
        }
    }

    // reconstruct
    fn reconstruct(&self, pos: usize) -> Vec<Edge> {
        //reconstruct the cycle
        let mut cycle = Vec::new();
        let mut v = pos;
        loop {
            cycle.push(self.g[v].clone());
            v = self.g[v].to;
            if v == pos {
                break;
            }
        }
        cycle
    }

    // find all cycle
    fn detect_all(&mut self) -> Vec<Vec<Edge>> {
        let mut res = Vec::new();
        for v in 0..self.g.len() {
            if self.finished[v] {
                continue;
            }
            if let Some(pos) = self.search(v) {
                let cycle = self.reconstruct(pos);
                if cycle.len() > 0 {
                    res.push(cycle)
                }
            }
        }
        res
    }
}

fn rec(v: usize, a: &Vec<usize>, dp: &mut Vec<Option<usize>>) -> usize {
    if let Some(x) = dp[v] {
        x
    } else {
        let x = rec(a[v], a, dp) + 1;
        dp[v] = Some(x);
        x
    }
}

// functional graph
fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    }

    let g = a
        .iter()
        .enumerate()
        .map(|(from, &to)| Edge::new(from, to, 1))
        .collect_vec();

    let mut cycle_detection = CycleDetection::new(g.clone());
    let cycles = cycle_detection.detect_all();

    let mut dp = vec![None; n];
    for cycle in &cycles {
        let l = cycle.len();
        for e in cycle {
            dp[e.to] = Some(l);
        }
    }
    for v in 0..n {
        match dp[v] {
            Some(_) => (),
            None => {
                rec(v, &a, &mut dp);
            }
        }
    }
    println!("{}", dp.into_iter().map(|o| o.unwrap_or(0)).sum::<usize>());
}
