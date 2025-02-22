use proconio::input;

fn main() {
    input! {
        _n: usize, x: usize, y: usize, z: usize,
    }
    println!(
        "{}",
        if x.min(y) < z && z < x.max(y) {
            "Yes"
        } else {
            "No"
        }
    );
}
