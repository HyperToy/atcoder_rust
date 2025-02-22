use itertools::Itertools;
use proconio::{input, marker::Chars};

const N: usize = 8;
const CS: [char; N] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

fn main() {
    input! {
        s: [Chars; N],
    }
    println!(
        "{}",
        (0..N)
            .cartesian_product(0..N)
            .find_or_first(|&(i, j)| s[i][j] == '*')
            .map(|(i, j)| format!("{}{}", CS[j], N - i))
            .unwrap()
    );
}
