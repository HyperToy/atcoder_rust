use proconio::input;

fn main() {
    input! {
        a: i32, b: i32,
    }
    let (a, b) = if a < b { (a, b) } else { (b, a) };
    println!(
        "{}",
        (2 * a - b..=2 * b - a)
            .filter(|&x| x * 2 == a + b || a * 2 == b + x || b * 2 == x + a)
            .count()
    );
}
