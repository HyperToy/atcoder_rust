use proconio::input;

fn main() {
    input! {
        n: usize, m: usize, x: i32, y: i32,
        xs: [i32; n],
        ys: [i32; m],
    }
    println!(
        "{}",
        if x.max(*xs.iter().max().unwrap()) < y.min(*ys.iter().min().unwrap()) {
            "No War"
        } else {
            "War"
        }
    );
}
