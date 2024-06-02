use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize, k: i64,
        a: [i64; n],
    }
    let sum = a.iter().sum::<i64>();
    if k > 0 {
        println!("Yes\n{}", a.into_iter().sorted().join(" "));
    } else {
        if sum >= k {
            println!("Yes\n{}", a.into_iter().sorted().rev().join(" "))
        } else {
            println!("No");
        }
    };
}
