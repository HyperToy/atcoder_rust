use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        sc: [(i64, i64); n],
    }
    let mut count = BTreeMap::new();
    for (s, c) in sc {
        *(count.entry(s).or_insert(0)) += c;
    }
    let mut answer = 0;
    while let Some((mut s, mut c)) = count.pop_first() {
        if c % 2 == 1 {
            answer += 1;
        }
        c /= 2;
        s *= 2;
        while c > 0 {
            if c % 2 == 1 {
                *(count.entry(s).or_insert(0)) += 1;
            }
            c /= 2;
            s *= 2;
        }
    }
    println!("{}", answer);
}
