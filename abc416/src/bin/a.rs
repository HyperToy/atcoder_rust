use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        _n: usize, l: Usize1, r: Usize1,
        s: Chars,
    }
    println!("{}", yes_no(s[l..=r].iter().all(|c| c == &'o')));
}

fn yes_no(b: bool) -> &'static str {
    if b {
        "Yes"
    } else {
        "No"
    }
}
