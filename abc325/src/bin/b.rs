use proconio::input;

fn main() {
    input! {
        n: usize,
        wx: [(i32, usize); n],
    }
    let mut answer = 0;
    for i in 0..24 {
        let mut now = 0;
        for (w, x) in &wx {
            let time = (i + x) % 24;
            if 9 <= time && time < 18 {
                now += w;
            }
        }
        answer = answer.max(now);
    }
    println!("{}", answer);
}
