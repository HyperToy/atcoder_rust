use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
    }
    let mut set = BTreeSet::new();
    for i in 1..=12 {
        let a = repunit(i);
        for j in 1..=i {
            let b = repunit(j);
            for k in 1..=j {
                let c = repunit(k);
                set.insert(a + b + c);
            }
        }
    }
    println!("{}", set.iter().nth(n - 1).unwrap());
}
fn repunit(k: i64) -> i64 {
    let mut res = 0;
    for _ in 0..k {
        res *= 10;
        res += 1;
    }
    res
}
