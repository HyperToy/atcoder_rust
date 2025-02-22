use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let mut answer = Vec::new();
    let mut flag = true;
    for c in s {
        match c {
            '|' => {
                flag = !flag;
            }
            _ => {
                if flag {
                    answer.push(c);
                }
            }
        }
    }
    println!("{}", answer.iter().join(""));
}
