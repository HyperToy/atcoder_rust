use proconio::input;

fn main() {
    input! {
        a: (i32, i32, i32, i32, i32, i32),
        b: (i32, i32, i32, i32, i32, i32),
    }
    println!(
        "{}",
        if overlap((a.0, a.3), (b.0, b.3))
            && overlap((a.1, a.4), (b.1, b.4))
            && overlap((a.2, a.5), (b.2, b.5))
        {
            "Yes"
        } else {
            "No"
        }
    );
}
fn overlap(a: (i32, i32), b: (i32, i32)) -> bool {
    a.0 < b.1 && b.0 < a.1
}
