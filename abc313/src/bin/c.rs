use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let sum = a.iter().sum::<usize>();
    let min = sum / n;
    let max = min + if sum % n == 0 { 0 } else { 1 };
    let mut answer = 0;
    let mut answer2 = 0;
    for x in a {
        if x < min {
            answer += min - x;
        }
        if x > max {
            answer2 += x - max;
        }
    }
    println!("{}", answer.max(answer2));
}
