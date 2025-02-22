use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    println!(
        "{}",
        [vec!['L'], vec!['o'; n], vec!['n', 'g']]
            .iter()
            .flatten()
            .join("")
    );
}
