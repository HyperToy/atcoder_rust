use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
        x: i32,
    }
    println!("{}", yes_no(a.iter().any(|&a| a == x)));
}

fn yes_no(b: bool) -> &'static str {
    if b {
        "Yes"
    } else {
        "No"
    }
}
