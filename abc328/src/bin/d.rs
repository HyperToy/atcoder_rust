use itertools::Itertools;
use proconio::{input, marker::Bytes};

fn main() {
    input! {
        s: Bytes
    }
    let mut answer = Vec::new();
    for c in s {
        match c {
            b'A' | b'B' => {
                answer.push(c);
            }
            b'C' => {
                if answer.len() >= 2 {
                    let n = answer.len();
                    if answer[n - 2] == b'A' && answer[n - 1] == b'B' {
                        answer.pop();
                        answer.pop();
                    } else {
                        answer.push(c);
                    }
                } else {
                    answer.push(c);
                }
            }
            _ => {
                unreachable!()
            }
        }
    }
    println!("{}", answer.into_iter().map(|c| c as char).join(""));
}
