use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    println!(
        "{}",
        if length(s[0], s[1]) == length(t[0], t[1]) {
            "Yes"
        } else {
            "No"
        }
    );
}
fn length(a: char, b: char) -> i32 {
    let a = digit(a);
    let b = digit(b);
    let (a, b) = if a < b { (a, b) } else { (b, a) };
    let diff = b - a;
    diff.min(5 - diff)
}
fn digit(a: char) -> i32 {
    match a {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        'D' => 3,
        'E' => 4,
        _ => unreachable!(),
    }
}
