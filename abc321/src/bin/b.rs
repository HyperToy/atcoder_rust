use proconio::*;

fn main() {
    input! {
        n: usize, x: i32,
        a: [i32; n - 1],
    }
    let max = a.clone().into_iter().max().unwrap();
    let min = a.clone().into_iter().min().unwrap();
    let sum = a.clone().into_iter().sum::<i32>();
    let mut answer = -1;
    for i in 0..=100 {
        let score = if i < min {
            sum - max
        } else if i < max {
            sum - min - max + i
        } else {
            sum - min
        };
        if score >= x {
            answer = i;
            break;
        }
    }
    println!("{}", answer);
}
