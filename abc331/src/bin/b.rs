use proconio::input;

fn main() {
    input! {
        n: i32, s: i32, m: i32, l: i32,
    }
    let mut answer = std::i32::MAX;
    for i in 0..=n / 6 + 1 {
        for j in 0..=n / 8 + 1 {
            for k in 0..=n / 12 + 1 {
                if i * 6 + j * 8 + k * 12 >= n {
                    answer = answer.min(s * i + m * j + l * k);
                }
            }
        }
    }
    println!("{}", answer);
}
