use proconio::input;
use std::collections::BTreeSet;

// grundy数 個数制限 Nim
fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }
    let mut set = BTreeSet::new();
    for x in a {
        if set.contains(&x) {
            set.remove(&x);
        } else {
            set.insert(x);
        }
    }
    println!(
        "{}",
        if set.len() == 0 {
            0
        } else if set.iter().fold(0, |xor_sum, x| xor_sum ^ *x) > 0 {
            -1
        } else {
            *set.last().unwrap() as isize - 1
        }
    );
}
