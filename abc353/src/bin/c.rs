use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let v = a.into_iter().sorted().collect::<Vec<_>>();
    let mut answer = 0;
    for i in 0..n {
        let x = v.lower_bound(&(100000000 - v[i])) as i64;
        answer += (x - if (i as i64) < x { 1 } else { 0 }) * v[i];
        answer += (n as i64 - x - if (i as i64) < x { 0 } else { 1 }) * (v[i] - 50000000);
    }
    println!("{}", answer);
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
