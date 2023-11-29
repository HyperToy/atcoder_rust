use proconio::input;

fn main() {
    input! {
        n: usize,
        d: [usize; n],
    }
    let mut answer = 0;
    for i in 1..=n {
        for j in 1..=d[i - 1] {
            if is_zorome(i, j) {
                answer += 1;
            }
        }
    }
    println!("{}", answer);
}

fn is_zorome(i: usize, j: usize) -> bool {
    i % 10 == j % 10 && (i < 10 || i / 10 == i % 10) && (j < 10 || j / 10 == j % 10)
}
