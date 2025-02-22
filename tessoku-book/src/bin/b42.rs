use proconio::*;

fn main() {
    input! {
        n: usize,
        cards: [(i64, i64); n],
    }
    let v = vec![-1, 1];
    let mut answer = 0;
    for a in &v {
        for b in &v {
            let mut front = 0;
            let mut back = 0;
            for (x, y) in &cards {
                if x * a > 0 && y * b > 0 || x * a + y * b > 0 {
                    front += x;
                    back += y;
                }
            }
            answer = answer.max(front.abs() + back.abs());
        }
    }
    println!("{}", answer);
}
