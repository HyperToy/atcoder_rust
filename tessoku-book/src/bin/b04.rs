use proconio::{marker::Chars, *};

fn main() {
    input! {
        n: Chars,
    }
    let mut answer = 0;
    for c in n {
        answer <<= 1;
        if c == '1' {
            answer |= 1;
        }
    }
    println!("{}", answer);
}
