use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        mut a: [Usize1; n],
    }
    let mut inv_a = vec![0; n];
    for i in 0..n {
        inv_a[a[i]] = i;
    }
    let mut answer = Vec::new();
    for i in 0..n {
        if a[i] == i {
            continue;
        }
        let j = inv_a[i];
        answer.push((i, j));
        a[j] = a[i];
        a[i] = i;
        inv_a[a[i]] = i;
        inv_a[a[j]] = j;
    }
    println!("{}", answer.len());
    if answer.len() > 0 {
        println!(
            "{}",
            answer
                .iter()
                .map(|(i, j)| format!("{} {}", i + 1, j + 1))
                .join("\n")
        );
    }
}
