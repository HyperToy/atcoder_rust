use proconio::*;

fn main() {
    input! {
        n: usize, l: i32,
        people: [(i32, char); n],
    }
    let mut answer = 0;
    for (a, b) in people {
        answer = answer.max(match b {
            'E' => l - a,
            'W' => a,
            _ => unreachable!(),
        });
    }
    println!("{}", answer);
}
