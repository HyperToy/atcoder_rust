use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize, m: usize, a: i32, b: i32,
        x: [Usize1; m],
    }
    let mut c = vec![0; n];
    for x in x {
        c[x] += 1;
    }
    c.sort();
    let mut answer = 0;
    let mut sum = 0;
    for c in c {
        sum += c;
        if sum <= b {
            answer += 1;
        }
    }
    println!("{}", answer.max(n as i32 - (a - b)));
}
