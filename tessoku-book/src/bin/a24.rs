use proconio::*;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    println!("{}", longest_increasing_subsequence(&a));
}

fn longest_increasing_subsequence<T: Ord + Copy>(a: &Vec<T>) -> usize {
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

#[test]
fn test_binary_search() {
    let vec = vec![1, 2, 4, 6, 7, 12, 54, 60];

    assert_eq!(vec.lower_bound(&4), 2);
    assert_eq!(vec.upper_bound(&4), 3);
}
