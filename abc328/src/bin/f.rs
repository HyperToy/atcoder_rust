use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize, q: usize,
        queries: [(Usize1, Usize1, i64); q],
    }
    let mut dsu = WeightedDsu::new(n);
    println!(
        "{}",
        queries
            .iter()
            .enumerate()
            .filter_map(|(i, &(a, b, d))| if dsu.merge(a, b, d) {
                Some(i + 1)
            } else {
                None
            })
            .join(" ")
    );
}

struct WeightedDsu {
    parent: Vec<usize>,
    weight: Vec<i64>,
}
impl WeightedDsu {
    fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect_vec(),
            weight: vec![0; size],
        }
    }
    fn root(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            return i;
        }
        let r = self.root(self.parent[i]);
        self.weight[i] = self.weight[self.parent[i]] + self.weight[i];
        self.parent[i] = r;
        r
    }
    fn merge(&mut self, i: usize, j: usize, x: i64) -> bool {
        let root_i = self.root(i);
        let root_j = self.root(j);
        if root_i == root_j {
            return self.weight[j] - self.weight[i] == x;
        }
        self.parent[root_i] = root_j;
        self.weight[root_i] = self.weight[j] - self.weight[i] - x;
        true
    }
}
