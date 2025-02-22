use proconio::*;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut stack: Vec<(usize, usize)> = Vec::new();
    for i in 0..n {
        while !stack.is_empty() && stack.last().unwrap().1 < a[i] {
            stack.pop();
        }
        println!(
            "{}",
            match stack.last() {
                None => -1,
                Some(x) => x.0 as isize,
            }
        );
        stack.push((i + 1, a[i]));
    }
}
