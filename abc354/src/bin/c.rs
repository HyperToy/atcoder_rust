use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        ac: [(i32, i32); n],
    }
    let ac = ac
        .into_iter()
        .enumerate()
        .sorted_by_key(|&(_, (a, _))| a)
        .collect::<Vec<_>>();
    let mut stack = Vec::new();
    for now in ac {
        while !stack.is_empty() {
            let top: &(usize, (i32, i32)) = stack.last().unwrap();
            if top.1 .1 > now.1 .1 {
                stack.pop();
            } else {
                break;
            }
        }
        stack.push(now);
    }
    println!("{}", stack.len());
    println!("{}", stack.iter().map(|(i, _)| i + 1).sorted().join(" "));
}
