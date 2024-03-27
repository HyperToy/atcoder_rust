use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let a = to_num(s[0]) * 10 + to_num(s[1]);
    let b = to_num(s[2]) * 10 + to_num(s[3]);
    println!(
        "{}",
        match (monthable(a), monthable(b)) {
            (true, true) => "AMBIGUOUS",
            (true, false) => "MMYY",
            (false, true) => "YYMM",
            (false, false) => "NA",
        }
    )
}
fn to_num(c: char) -> u8 {
    c as u8 - b'0'
}
fn monthable(x: u8) -> bool {
    1 <= x && x <= 12
}
