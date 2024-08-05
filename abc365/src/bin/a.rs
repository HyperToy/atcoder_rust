use proconio::input;

fn main() {
    input! {
        year: usize,
    }
    println!(
        "{}",
        match year {
            n if n % 400 == 0 => 366,
            n if n % 100 == 0 => 365,
            n if n % 4 == 0 => 366,
            _ => 365,
        }
    );
}
