use proconio::{marker::Chars, *};

fn main() {
    input! {
        _n: usize, c: char,
        s: Chars,
    }
    let sum = s
        .into_iter()
        .map(|x| match x {
            'W' => 0,
            'R' => 1,
            'B' => 2,
            _ => unreachable!(),
        })
        .sum::<i32>();
    let target = match c {
        'W' => 0,
        'R' => 1,
        'B' => 2,
        _ => unreachable!(),
    };
    println!("{}", if sum % 3 == target { "Yes" } else { "No" });
}
