use proconio::*;

fn main() {
    input! {
        n: usize, _h: usize, _w: usize,
        pieces: [(usize, usize); n],
    }
    let mut xor = 0;
    for (a, b) in pieces {
        xor ^= a - 1;
        xor ^= b - 1;
    }
    println!("{}", if xor == 0 { "Second" } else { "First" });
}
