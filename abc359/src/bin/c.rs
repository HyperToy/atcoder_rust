use proconio::input;

fn main() {
    input! {
        s: (i64, i64),
        t: (i64, i64),
    }
    let s = (s.0 - (s.0 + s.1) % 2, s.1);
    let t = (t.0 - (t.0 + t.1) % 2, t.1);
    let dy = (t.1 - s.1).abs();
    let dx = (t.0 - s.0).abs();
    let answer = dy + (0.max(dx - dy)) / 2;
    println!("{}", answer);
}
