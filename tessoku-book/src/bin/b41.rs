use proconio::*;

fn main() {
    input! {
        mut x: i32, mut y: i32,
    }
    let mut answer = Vec::new();
    while x > 1 || y > 1 {
        answer.push((x, y));
        if x > y {
            x -= y;
        } else {
            y -= x;
        }
    }
    answer.reverse();
    println!("{}", answer.len());
    for (x, y) in answer {
        println!("{} {}", x, y);
    }
}
