use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
    }
    let mut answer = Vec::new();
    let mut count = 0;
    for c in s {
        match c {
            '(' => {
                count += 1;
                answer.push(c);
            }
            ')' => {
                if count > 0 {
                    count -= 1;
                    while let Some(x) = answer.pop() {
                        if x == '(' {
                            break;
                        }
                    }
                } else {
                    answer.push(c);
                }
            }
            _ => answer.push(c),
        }
    }
    println!("{}", answer.iter().join(""));
}
