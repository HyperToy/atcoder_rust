use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize, x: i64,
        a: [i64; n],
    }
    let s = a.clone().into_iter().collect::<BTreeSet<_>>();
    println!(
        "{}",
        if a.into_iter().any(|a| s.contains(&(a + x))) {
            "Yes"
        } else {
            "No"
        }
    );
}
