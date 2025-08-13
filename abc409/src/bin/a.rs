use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        t: Chars,
        a: Chars,
    }
    print_yes_no(t.iter().zip(a.iter()).any(|(t, a)| t == a && t == &'o'));
}

fn print_yes_no(b: bool) {
    println!("{}", if b { "Yes" } else { "No" });
}
