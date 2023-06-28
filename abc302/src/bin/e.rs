use proconio::{marker::Usize1, *};
use std::collections::HashSet;

fn main() {
    input! {
        n: usize, q: usize,
    }
    let mut sets = vec![HashSet::new(); n];
    let mut connected = 0;
    for _ in 0..q {
        input! {
            t: i32,
        }
        if t == 1 {
            input! {
                u: Usize1, v: Usize1,
            }
            connected += if sets[u].len() == 0 { 1 } else { 0 };
            sets[u].insert(v);
            connected += if sets[v].len() == 0 { 1 } else { 0 };
            sets[v].insert(u);
        }
        if t == 2 {
            input! {
                u: Usize1,
            }
            connected -= if sets[u].len() > 0 { 1 } else { 0 };
            let s = sets[u].clone();
            for v in s {
                sets[v].remove(&u);
                connected -= if sets[v].len() == 0 { 1 } else { 0 };
            }
            sets[u] = HashSet::new();
        }
        println!("{}", n - connected);
    }
}
