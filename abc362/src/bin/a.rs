use proconio::input;

fn main() {
    input! {
        r: i32, g: i32, b: i32,
        c: String
    }
    println!(
        "{}",
        match c.as_str() {
            "Red" => g.min(b),
            "Green" => b.min(r),
            "Blue" => r.min(g),
            _ => unreachable!(),
        }
    );
}
