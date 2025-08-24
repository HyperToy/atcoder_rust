use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        m: u32,
    }
    let answer = (0..=10)
        .rev()
        .fold((vec![], m), |(mut v, mut m), i| {
            let p = 3_u32.pow(i);
            while m >= p {
                m -= p;
                v.push(i);
            }
            (v, m)
        })
        .0;
    println!("{}\n{}", answer.len(), answer.iter().join(" "));
}
