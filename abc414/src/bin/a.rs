use proconio::input;

fn main() {
    input! {
        n: usize, l: i32, r: i32,
        ranges: [(i32, i32); n],
    }
    println!(
        "{}",
        ranges
            .into_iter()
            .filter(|(x, y)| *x <= l && *y >= r)
            .count()
    );
}
