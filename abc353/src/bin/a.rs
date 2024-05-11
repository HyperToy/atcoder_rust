use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [i32; n],
    }
    println!(
        "{}",
        h.iter()
            .skip(1)
            .position(|&x| x > h[0])
            .map(|x| x as isize + 2)
            .unwrap_or(-1)
    );
}
