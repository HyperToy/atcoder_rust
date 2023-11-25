use proconio::*;
use std::collections::{HashMap, VecDeque};

fn main() {
    input! {
        (w, h): (i32, i32),
        n: usize,
        pq: [(i32, i32); n],
        a_len: usize,
        mut a: [i32; a_len],
        b_len: usize,
        mut b: [i32; b_len],
    }
    let mut a = VecDeque::from(a);
    a.push_front(0);
    a.push_back(w);
    let mut b = VecDeque::from(b);
    b.push_front(0);
    b.push_back(h);

    let mut count = HashMap::new();
    pq.iter()
        .map(|(p, q)| (a.lower_bound(p), b.lower_bound(q)))
        .for_each(|key| {
            *(count.entry(key).or_insert(0)) += 1;
        });

    let mut answer_min = if count.len() < (a_len + 1) * (b_len + 1) {
        0
    } else {
        n
    };
    let mut answer_max = 0;
    for (_, val) in count {
        if val > answer_max {
            answer_max = val;
        }
        if val < answer_min {
            answer_min = val;
        }
    }
    println!("{} {}", answer_min, answer_max);
}

pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}

impl<T: Ord> BinarySearch<T> for VecDeque<T> {
    fn lower_bound(&self, x: &T) -> usize {
        let mut ng = 0;
        let mut ok = self.len();
        while ng + 1 < ok {
            let wj = (ng + ok) / 2;
            if self[wj] >= *x {
                ok = wj;
            } else {
                ng = wj;
            }
        }
        ok
    }

    fn upper_bound(&self, x: &T) -> usize {
        let mut ng = 0;
        let mut ok = self.len();
        while ng + 1 < ok {
            let wj = (ng + ok) / 2;
            if self[wj] > *x {
                ok = wj;
            } else {
                ng = wj;
            }
        }
        ok
    }
}

#[test]
fn test_binary_search() {
    let vec = VecDeque::from([1, 2, 4, 6, 7, 12, 54, 60]);

    assert_eq!(vec.lower_bound(&4), 2);
    assert_eq!(vec.upper_bound(&4), 3);
}
