use proconio::*;

fn main() {
    input! {
        n: usize,
        s: String,
    }
    let mut t = 0;
    let mut a = 0;
    for c in s.chars() {
        match c {
            'T' => t += 1,
            'A' => a += 1,
            _ => unreachable!(),
        }
        if t == (n + 1) / 2 {
            println!("T");
            return;
        }
        if a == (n + 1) / 2 {
            println!("A");
            return;
        }
    }
}
