use proconio::input;

fn main() {
    input! {
        ps: [(i32, i32); 3],
    }
    println!(
        "{}",
        if (0..3).any(|i| {
            let j = (i + 1) % 3;
            let k = (i + 2) % 3;
            culc(diff(ps[i], ps[j]), diff(ps[i], ps[k]))
        }) {
            "Yes"
        } else {
            "No"
        }
    );
}
fn diff(p: (i32, i32), q: (i32, i32)) -> (i32, i32) {
    (q.0 - p.0, q.1 - p.1)
}
fn culc(p: (i32, i32), q: (i32, i32)) -> bool {
    p.0 * q.0 + p.1 * q.1 == 0
}
