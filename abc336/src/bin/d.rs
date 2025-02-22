use proconio::input;

// todo: refactor
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut forward = (1..=n).collect::<Vec<_>>();
    for i in 1..n {
        forward[i] = forward[i].min(a[i]).min(forward[i - 1] + 1);
    }
    let mut reverse = (1..=n).rev().collect::<Vec<_>>();
    for i in (0..n - 1).rev() {
        reverse[i] = reverse[i].min(a[i]).min(reverse[i + 1] + 1);
    }
    let mut answer = 0;
    for i in 0..n {
        answer = answer.max(std::cmp::min(forward[i], reverse[i]));
    }
    println!("{}", answer);
}
