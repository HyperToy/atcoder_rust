use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [usize; n],
    }
    let mut stack = Vec::new();
    stack.push((std::usize::MAX, 0));
    let mut answer = Vec::new();
    answer.push(0);
    for i in 1..=n {
        let now_h = h[i - 1];
        while stack.last().is_some_and(|&(h, _)| now_h >= h) {
            stack.pop();
        }
        let idx = stack.last().unwrap().1;
        answer.push(answer[idx] + now_h * (i - idx));
        stack.push((now_h, i));
    }
    println!("{}", answer.into_iter().skip(1).map(|x| x + 1).join(" "));
}
