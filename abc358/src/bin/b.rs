use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize, a: i32,
        t: [i32; n],
    }
    let mut time = 0;
    let mut answer = vec![];
    for t in t {
        time = time.max(t);
        time += a;
        answer.push(time);
    }
    println!("{}", answer.iter().join("\n"));
}
