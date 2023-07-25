use proconio::{marker::Chars, *};

fn main() {
    input! {
        s: Chars,
    }
    let mut joi = 0;
    let mut ioi = 0;
    for i in 2..s.len() {
        if s[i - 1] == 'O' && s[i] == 'I' {
            if s[i - 2] == 'J' {
                joi += 1;
            }
            if s[i - 2] == 'I' {
                ioi += 1;
            }
        }
    }
    println!("{}", joi);
    println!("{}", ioi);
}
