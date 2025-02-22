use proconio::input;

fn main() {
    input! {
        m: usize,
        d: [usize; m],
    }
    let sum = d.iter().sum::<usize>();
    let mut mid = (sum + 1) / 2;
    for i in 1..=m {
        if mid > d[i - 1] {
            mid -= d[i - 1];
        } else {
            println!("{} {}", i, mid);
            break;
        }
    }
}
