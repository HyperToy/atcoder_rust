use proconio::*;

// todo
fn main() {
    input! {
        n: usize,
        mut problems: [(i32, i32); n],
    }
    let mut answer = 0;
    let mut time = 0;
    problems.sort_by(|a, b| a.1.cmp(&b.1));
    for (t, d) in problems {
        if time + t <= d {
            time += t;
            answer += 1;
        }
    }
    println!("{}", answer);
}
