use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize, m: usize,
        c: [String; n],
        d: [String; m],
        p: [i32; m + 1],
    }
    let mut price = HashMap::new();
    for j in 0..m {
        price.insert(d[j].clone(), p[j + 1]);
    }
    println!(
        "{}",
        c.into_iter()
            .map(|s| if price.contains_key(&s) {
                *price.get(&s).unwrap()
            } else {
                p[0]
            })
            .sum::<i32>()
    );
}
