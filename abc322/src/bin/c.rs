use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize, m: usize,
        a: [Usize1; m],
    }
    for i in 0..n {
        println!("{}", a[a.lower_bound(&i)] - i);
    }
}

pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}

impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        let mut ng = -1;
        let mut ok = self.len() as isize;
        while ng + 1 < ok {
            let wj = (ng + ok) / 2;
            if self[wj as usize] >= *x {
                ok = wj;
            } else {
                ng = wj;
            }
        }
        ok as usize
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
