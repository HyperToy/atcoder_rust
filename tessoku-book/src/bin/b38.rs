use proconio::{marker::Chars, *};

fn main() {
    input! {
        n: usize,
        s: Chars
    }
    let mut heights = vec![1; n];
    for i in 0..n - 1 {
        if s[i] == 'A' {
            heights[i + 1] = heights[i + 1].max(heights[i] + 1);
        }
    }
    for i in (0..n - 1).rev() {
        if s[i] == 'B' {
            heights[i] = heights[i].max(heights[i + 1] + 1);
        }
    }
    println!("{}", heights.iter().sum::<i32>());
}
