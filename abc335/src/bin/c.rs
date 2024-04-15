use itertools::Itertools;
use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: i32, q: usize,
        queries: [(u8, String); q],
    }
    let mut v = VecDeque::new();
    for i in 0..n {
        v.push_back((0, i + 1));
    }
    let mut answer = Vec::new();
    for (t, x) in queries {
        match t {
            1 => {
                let head = v.front().unwrap();
                let d = match x.as_str() {
                    "R" => (0, 1),
                    "L" => (0, -1),
                    "U" => (1, 0),
                    "D" => (-1, 0),
                    _ => unreachable!(),
                };
                let nx = (head.0 + d.0, head.1 + d.1);
                v.push_front(nx);
                v.pop_back();
            }
            2 => {
                let i = x.parse::<usize>().unwrap() - 1;
                answer.push(v[i]);
            }
            _ => unreachable!(),
        }
    }
    println!(
        "{}",
        answer
            .iter()
            .map(|(y, x)| format!("{} {}", x, y))
            .join("\n")
    );
}
