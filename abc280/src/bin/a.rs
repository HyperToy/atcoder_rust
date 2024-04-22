use proconio::{input, marker::Chars};

fn main() {
    input! {
        (h, _w): (usize, usize),
        s: [Chars; h],
    }
    println!("{}", s.iter().flatten().filter(|c| **c == '#').count());
}
