use proconio::*;
use std::cmp::Ordering;

fn main() {
    input! {
        n: usize,
        mut boxes: [(usize, usize); n],
    }
    boxes.sort_by(|a, b| match a.0.cmp(&b.0) {
        Ordering::Greater | Ordering::Less => a.0.cmp(&b.0),
        Ordering::Equal => b.1.cmp(&a.1),
    });
    println!("{}", longest_increasing_subsequence(&boxes));
}

fn longest_increasing_subsequence<T: Ord + Copy>(a: &Vec<(T, T)>) -> usize {
    let n = a.len();
    let mut lis = Vec::new();
    for i in 0..n {
        let pos = lis.lower_bound(&a[i]);
        if pos < lis.len() {
            lis[pos] = a[i];
        } else {
            lis.push(a[i]);
        }
    }
    lis.len()
}

/// Equivalent to std::lower_bound and std::upper_bound in c++
pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &(T, T)) -> usize;
}

impl<T: Ord> BinarySearch<T> for [(T, T)] {
    fn lower_bound(&self, x: &(T, T)) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            // match self[mid].cmp(x) {
            //     Ordering::Less => {
            //         low = mid + 1;
            //     }
            //     Ordering::Equal | Ordering::Greater => {
            //         high = mid;
            //     }
            // }
            if self[mid].0 < x.0 && self[mid].1 < x.1 {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        low
    }
}
