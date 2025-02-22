use itertools::Itertools;
use proconio::{
    marker::{Chars, Usize1},
    *,
};
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize, x: Usize1,
        mut a: Chars,
    }
    let mut queue = VecDeque::new();
    queue.push_back(x);
    a[x] = '@';
    while queue.len() > 0 {
        let pos = queue.pop_front().unwrap();
        if pos > 0 && a[pos - 1] == '.' {
            queue.push_back(pos - 1);
            a[pos - 1] = '@';
        }
        if pos < n - 1 && a[pos + 1] == '.' {
            queue.push_back(pos + 1);
            a[pos + 1] = '@';
        }
    }
    println!("{}", a.iter().join(""));
}
