use proconio::*;
use std::collections::BTreeSet;

fn main() {
    input! {
        q: usize,
    }
    let mut set = BTreeSet::new();
    for _ in 0..q {
        input! {
            t: usize, x: i32,
        }
        match t {
            1 => {
                set.insert(x);
            }
            2 => {
                set.remove(&x);
            }
            3 => {
                println!(
                    "{}",
                    match set.range(x..).next() {
                        Some(x) => x,
                        None => &-1,
                    }
                )
            }
            _ => unreachable!(),
        }
    }
}
