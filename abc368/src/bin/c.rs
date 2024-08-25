use proconio::input;

fn main() {
    input! {
        n: usize,
        hs: [u64; n],
    }
    let mut answer = 0;
    for h in hs {
        answer += h / 5 * 3;
        answer += match (answer % 3, h % 5) {
            (_, 0) => 0,
            (_, 1) => 1,
            (0, 2) => 2,
            (1, 2) => 2,
            (2, 2) => 1,
            (0, 3) => 3,
            (1, 3) => 2,
            (2, 3) => 1,
            (0, 4) => 3,
            (1, 4) => 2,
            (2, 4) => 2,
            (_, _) => unreachable!(),
        };
    }
    println!("{}", answer);
}
