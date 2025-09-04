use ac_library::Dsu;
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize, q: usize,
        qs: [(usize, Usize1, Usize1); q],
    }
    let mut dsu = MyDsu::new(n, 10);
    println!(
        "{}",
        qs.into_iter()
            .filter_map(|q| match q {
                (1, u, v) => {
                    dsu.merge(u, v);
                    None
                }
                (2, v, k) => {
                    Some(dsu.query(v, k))
                }
                _ => unreachable!(),
            })
            .map(|x| match x {
                Some(x) => (x + 1).to_string(),
                None => "-1".to_string(),
            })
            .join("\n")
    );
}

struct MyDsu {
    dsu: Dsu,
    reachable: Vec<Vec<usize>>,
    max: usize,
}
impl MyDsu {
    fn new(n: usize, max: usize) -> Self {
        Self {
            dsu: Dsu::new(n),
            reachable: (0..n).map(|i| vec![i]).collect(),
            max,
        }
    }
    fn merge(&mut self, a: usize, b: usize) {
        if self.dsu.same(a, b) {
            return;
        }
        let reachable_a = std::mem::replace(&mut self.reachable[self.dsu.leader(a)], vec![]);
        let reachable_b = std::mem::replace(&mut self.reachable[self.dsu.leader(b)], vec![]);
        let new_reachable = reachable_a
            .into_iter()
            .chain(reachable_b)
            .sorted()
            .rev()
            .take(self.max)
            .collect::<Vec<_>>();
        self.dsu.merge(a, b);
        let new_leader = self.dsu.leader(a);
        self.reachable[new_leader] = new_reachable;
    }
    fn query(&mut self, v: usize, k: usize) -> Option<usize> {
        let leader = self.dsu.leader(v);
        self.reachable[leader].get(k).copied()
    }
}
