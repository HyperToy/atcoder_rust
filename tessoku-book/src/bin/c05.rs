use proconio::*;

fn main() {
    input! {
        n: usize,
    }
    println!(
        "{}",
        format!("{:010b}", n - 1)
            .replace("0", "4")
            .replace("1", "7")
    );
}
