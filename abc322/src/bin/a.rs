use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut answer = -1;
    for i in 0..n - 2 {
        if answer == -1 && (s[i], s[i + 1], s[i + 2]) == ('A', 'B', 'C') {
            answer = i as isize + 1;
        }
    }
    println!("{}", answer);
}
