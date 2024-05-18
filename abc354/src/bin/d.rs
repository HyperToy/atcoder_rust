use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        d: i64,
    }
    let x_move = if a < 0 { (-a + 3) / 4 * 4 } else { 0 };
    let y_move = if b < 0 { (-b + 1) / 2 * 2 } else { 0 };
    let (a, c) = (a + x_move, c + x_move);
    let (b, d) = (b + y_move, d + y_move);
    println!("{}", calc(c, d) - calc(a, d) - calc(c, b) + calc(a, b));
}

fn calc(x: i64, y: i64) -> i64 {
    let mut res = 0;
    let (xq, xr) = (x / 4, x % 4);
    let (yq, yr) = (y / 2, y % 2);
    res += xq * yq * 8;
    res += yq
        * match xr {
            1 => 3,
            2 => 6,
            3 => 7,
            _ => 0,
        };
    res += xq * yr * 4;
    res += yr
        * match xr {
            1 => 2,
            2 => 3,
            3 => 3,
            _ => 0,
        };
    res
}
