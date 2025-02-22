use proconio::input;

fn main() {
    input! {
        n: usize,
        q: [i32; n],
        a: [i32; n],
        b: [i32; n],
    }
    let mut x = (0..n)
        .map(|i| {
            if a[i] == 0 {
                std::i32::MAX
            } else {
                q[i] / a[i]
            }
        })
        .min()
        .unwrap();
    let mut y = (0..n)
        .map(|i| {
            if b[i] == 0 {
                std::i32::MAX
            } else {
                (q[i] - a[i] * x) / b[i]
            }
        })
        .min()
        .unwrap();
    let mut answer = x + y;
    // refactor
    while x > 0 {
        x -= 1;
        y = (0..n)
            .map(|i| {
                if b[i] == 0 {
                    std::i32::MAX
                } else {
                    (q[i] - a[i] * x) / b[i]
                }
            })
            .min()
            .unwrap();
        answer = answer.max(x + y);
    }
    println!("{}", answer);
}
