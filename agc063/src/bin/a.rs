use itertools::Itertools;
use proconio::{input, marker::Chars};

// todo refactor
fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let sum_a = sum(&s, 'A');
    let sum_b = sum(&s, 'B');
    let mut answer = Vec::new();
    let mut a_pos = 0;
    let mut b_pos = 0;
    for k in 1..=n {
        let a_count = (k + 1) / 2;
        let b_count = k / 2;
        while sum_b[a_pos + 1] <= a_count {
            a_pos += 1;
        }
        while sum_a[b_pos + 1] <= b_count {
            b_pos += 1;
        }
        answer.push(if a_pos > b_pos { "Alice" } else { "Bob" });
    }
    println!("{}", answer.iter().join("\n"));
}
fn sum(v: &Vec<char>, t: char) -> Vec<usize> {
    let mut res = vec![0; v.len() + 1];
    for i in 0..v.len() {
        res[i + 1] = res[i] + if v[i] == t { 1 } else { 0 };
    }
    res.push(std::usize::MAX); // 番兵
    res
}
