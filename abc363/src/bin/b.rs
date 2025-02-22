use proconio::input;

fn main() {
    input! {
        n: usize, t: i32, p: usize,
        l: [i32; n],
    }
    println!(
        "{}",
        (0..)
            .find(|&i| { l.iter().filter(|&l| l + i >= t).count() >= p })
            .unwrap()
    );
}
