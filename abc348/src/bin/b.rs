use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        ps: [(i32, i32); n],
    }
    let mut answer = vec![];
    for (x, y) in &ps {
        let mut now = n;
        let mut max = 0;
        for j in 0..n {
            let dist = (*x - ps[j].0).pow(2) + (*y - ps[j].1).pow(2);
            if max < dist {
                max = dist;
                now = j;
            }
        }
        answer.push(now + 1);
    }
    println!("{}", answer.iter().join("\n"));
}
