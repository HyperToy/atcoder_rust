use proconio::input;

fn main() {
    input! {
        x: i64, y: i64,
    }
    println!(
        "{}",
        solve(x, y).map_or("-1".to_string(), |(a, b)| format!("{} {}", a, b))
    );
}
fn solve(x: i64, y: i64) -> Option<(i64, i64)> {
    let dx = if x < 0 { -1 } else { 1 };
    let dy = if y < 0 { -1 } else { 1 };
    let (x, y) = (x * dx, y * dy);
    let g = gcd(x, y);
    if g > 2 {
        return None;
    }
    let (a, b) = ext_gcd(x, y);
    Some((b * dy * 2 / g, -a * dx * 2 / g))
}
fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
// ax + by = ± gsd(a,b) となる (x,y) を求める
fn ext_gcd(a: i64, b: i64) -> (i64, i64) {
    if b == 0 {
        (1, 0)
    } else {
        let (y, x) = ext_gcd(b, a % b);
        let y = y - a / b * x;
        (x, y)
    }
}
