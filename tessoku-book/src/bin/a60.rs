use proconio::*;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut stack: Vec<(usize, usize)> = Vec::new();
    for i in 0..n {
        if stack.is_empty() {
            println!("{}", -1);
        } else {
            while stack.last().unwrap().1 < a[i] {
                stack.pop();
            }
            println!("{}", stack.last().unwrap().0);
        }
        stack.push((i + 1, a[i]));
    }
}
