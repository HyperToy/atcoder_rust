use proconio::input;

fn main() {
    input! {
        p: String,
        l: usize,
    }
    println!("{}", yes_no(p.len() >= l));
}

fn yes_no(b: bool) -> &'static str {
    if b {
        "Yes"
    } else {
        "No"
    }
}
