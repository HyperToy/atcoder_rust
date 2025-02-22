use proconio::*;

fn main() {
    input! {
        n: usize, k: i32,
        students: [(i32, i32); n],
    }
    let mut answer = 0;
    for i in 0..=100 {
        for j in 0..=100 {
            let mut now = 0;
            for (a, b) in &students {
                if i <= *a && *a <= i + k && j <= *b && *b <= j + k {
                    now += 1;
                }
            }
            answer = answer.max(now);
        }
    }
    println!("{}", answer);
}
