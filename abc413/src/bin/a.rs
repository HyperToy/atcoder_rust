use proconio::input;

fn main() {
    input! {
        n: usize, m: i32,
        a: [i32; n],
    }
    println!("{}", yes_no(a.into_iter().sum::<i32>() <= m));
}

fn yes_no(b: bool) -> &'static str {
    if b {
        "Yes"
    } else {
        "No"
    }
}
