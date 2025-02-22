use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars
    }
    let mut count = 0;
    let mut answer: i64 = 0;
    for c in s {
        match c {
            '>' => {
                count += 1;
            }
            '<' => {
                answer += count * (count + 1) / 2;
                count = 0;
            }
            _ => unreachable!(),
        }
    }
    answer += count * (count + 1) / 2;
    println!("{}", answer);
}
