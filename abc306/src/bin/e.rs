use itertools::Itertools;
use proconio::{input, marker::Usize1};
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize, k: usize, q: usize,
        queries: [(Usize1, i64); q],
    }
    let mut s = S::new(n, k);
    let mut answer = Vec::new();
    for (x, y) in queries {
        answer.push(s.query(x, y));
    }
    println!("{}", answer.iter().join(" "));
}

#[derive(Debug)]
struct S {
    a: Vec<i64>,
    k: usize,
    sum: i64,
    x_count: usize,
    x: BTreeMap<i64, u32>,
    y: BTreeMap<i64, u32>,
}
impl S {
    fn new(n: usize, k: usize) -> Self {
        let a = vec![0; n];
        let mut x = BTreeMap::new();
        let mut y = BTreeMap::new();
        for i in 0..=n {
            if i < k {
                *(x.entry(0).or_insert(0)) += 1;
            } else {
                // 番兵として y には 0 を 1つ多く追加
                *(y.entry(0).or_insert(0)) += 1;
            }
        }
        Self {
            a,
            k,
            sum: 0,
            x_count: k,
            x,
            y,
        }
    }
    fn balance(&mut self) {
        while self.x_count < self.k {
            // y の max を x に追加し、 sum を更新
            let max: i64 = *self.y.keys().max().unwrap();
            *(self.y.entry(max).or_default()) -= 1;
            if self.y.get(&max).unwrap() == &0 {
                self.y.remove(&max);
            }
            *(self.x.entry(max).or_insert(0)) += 1;
            self.sum += max;
            self.x_count += 1;
        }
        while self.x.keys().min().unwrap() < self.y.keys().max().unwrap() {
            // x の min と y の max を交換し、 sum を更新
            let max = *self.y.keys().max().unwrap();
            let min = *self.x.keys().min().unwrap();

            *(self.y.entry(max).or_default()) -= 1;
            if self.y.get(&max).unwrap() == &0 {
                self.y.remove(&max);
            }
            *(self.x.entry(max).or_insert(0)) += 1;
            self.sum += max;

            *(self.x.entry(min).or_default()) -= 1;
            if self.x.get(&min).unwrap() == &0 {
                self.x.remove(&min);
            }
            *(self.y.entry(min).or_insert(0)) += 1;
            self.sum -= min;
        }
    }
    fn add(&mut self, a: i64) {
        *(self.y.entry(a).or_insert(0)) += 1;
        self.balance();
    }
    fn erase(&mut self, a: i64) {
        if self.x.contains_key(&a) {
            // x にあれば x から削除して sum を更新
            *(self.x.entry(a).or_default()) -= 1;
            if self.x.get(&a).unwrap() == &0 {
                self.x.remove(&a);
            }
            self.x_count -= 1;
            self.sum -= a;
        } else if self.y.contains_key(&a) {
            // y にあれば y から削除
            *(self.y.entry(a).or_default()) -= 1;
            if self.y.get(&a).unwrap() == &0 {
                self.y.remove(&a);
            }
        } else {
            unreachable!();
        }
        self.balance();
    }
    fn query(&mut self, x: usize, y: i64) -> i64 {
        self.add(y);
        self.erase(self.a[x]);
        self.a[x] = y;
        self.sum
    }
}
