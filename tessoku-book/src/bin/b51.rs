use proconio::{marker::Chars, *};

fn main() {
    input! {
        s: Chars,
    }
    let mut stack = Vec::new();
    for i in 0..s.len() {
        match s[i] {
            '(' => {
                stack.push(i + 1);
            }
            ')' => {
                println!("{} {}", stack.pop().unwrap(), i + 1);
            }
            _ => unreachable!(),
        }
    }
}
