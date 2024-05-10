use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut v = Vec::new();
    for _ in 0..n {
        input! {
            c: usize, a: [i32; c],
        }
        v.push(a);
    }
    input! {
        x: i32,
    }
    let count = v
        .iter()
        .map(|v| {
            if v.contains(&x) {
                v.len()
            } else {
                std::usize::MAX
            }
        })
        .min()
        .unwrap_or(0);
    let iter = v.iter().enumerate().filter_map(|(i, v)| {
        if v.contains(&x) && v.len() == count {
            Some(i + 1)
        } else {
            None
        }
    });
    println!("{}\n{}", iter.clone().count(), iter.clone().join(" "));
}
