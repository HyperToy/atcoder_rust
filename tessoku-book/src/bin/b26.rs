use itertools::Itertools;
use proconio::*;

fn main() {
    input! {
        n: usize,
    }
    let prime_numbers = make_prime_numbers(n);
    println!("{}", prime_numbers.iter().join(" "));
}
fn make_prime_numbers(max: usize) -> Vec<usize> {
    let mut deleted = vec![false; max + 1];
    deleted[0] = true;
    deleted[1] = true;
    let mut i = 2;
    while i * i <= max {
        if deleted[i] {
            i += 1;
            continue;
        }
        let mut j = i * 2;
        while j <= max {
            deleted[j] = true;
            j += i;
        }
        i += 1;
    }
    let mut res = Vec::new();
    for i in 0..=max {
        if !deleted[i] {
            res.push(i);
        }
    }
    res
}
