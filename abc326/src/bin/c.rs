use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize, m: i32,
        mut a: [i32; n],
    }
    a.sort();
    let mut queue = VecDeque::new();
    let mut answer = 1;
    queue.push_back(a[0]);
    for i in 1..n {
        while !queue.is_empty() {
            if queue.front().unwrap() + m <= a[i] {
                queue.pop_front();
            } else {
                break;
            }
        }
        queue.push_back(a[i]);
        answer = answer.max(queue.len());
    }
    println!("{}", answer);
}
