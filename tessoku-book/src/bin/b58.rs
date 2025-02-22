use proconio::*;

fn main() {
    input! {
        n: usize, l: i32, r: i32,
        x: [i32; n],
    }
    let mut dp = vec![2_000_000_000; n];
    let mut rmq = SegTree::new(n, 2_000_000_000);
    dp[0] = 0;
    rmq.update(0, 0);
    for i in 1..n {
        let pos_l = x.lower_bound(&(x[i] - r));
        let pos_r = x.upper_bound(&(x[i] - l));
        dp[i] = rmq.query(pos_l, pos_r) + 1;
        rmq.update(i, dp[i]);
    }
    println!("{}", dp[n - 1]);
}

use std::cmp::Ordering;

/// Equivalent to std::lower_bound and std::upper_bound in c++
pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}

impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less => {
                    low = mid + 1;
                }
                Ordering::Equal | Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }

    fn upper_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less | Ordering::Equal => {
                    low = mid + 1;
                }
                Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }
}

struct SegTree {
    data: Vec<i32>,
    size: usize,
}
impl SegTree {
    fn new(n: usize, init_val: i32) -> Self {
        let mut size = 1;
        while size < n {
            size *= 2;
        }
        let data = vec![init_val; size * 2];
        Self { data, size }
    }
    fn update(&mut self, pos: usize, val: i32) {
        let mut pos = pos + self.size;
        self.data[pos] = val;
        while pos > 1 {
            pos /= 2;
            self.data[pos] = self.data[pos * 2].min(self.data[pos * 2 + 1]);
        }
    }
    fn query_sub(&self, l: usize, r: usize, a: usize, b: usize, k: usize) -> i32 {
        if r <= a || b <= l {
            return 2_000_000_000;
        }
        if l <= a && b <= r {
            return self.data[k];
        }
        let m = (a + b) / 2;
        let val_l = self.query_sub(l, r, a, m, k * 2);
        let val_r = self.query_sub(l, r, m, b, k * 2 + 1);
        val_l.min(val_r)
    }
    fn query(&self, l: usize, r: usize) -> i32 {
        self.query_sub(l, r, 0, self.size, 1)
    }
}
