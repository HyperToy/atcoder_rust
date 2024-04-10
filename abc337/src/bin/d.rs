use proconio::{input, marker::Chars};

fn main() {
    input! {
        (h, w): (usize, usize), k: usize,
        s: [Chars; h],
    }
    let mut answer = std::i32::MAX;
    for i in 0..h {
        // 横方向
        // (., o, x) のそれぞれの数の累積和をとる
        let mut sum = vec![(0, 0, 0); w + 1];
        for j in 0..w {
            sum[j + 1] = sum[j];
            match s[i][j] {
                '.' => {
                    sum[j + 1].0 += 1;
                }
                'o' => {
                    sum[j + 1].1 += 1;
                }
                'x' => {
                    sum[j + 1].2 += 1;
                }
                _ => unreachable!(),
            }
        }
        // 幅k の各区間について、 x がないもののうち、 . の数の最小をとる
        for l in 0..w {
            // (l, r] の範囲で考える
            let r = l + k;
            if r > w {
                break;
            }
            if sum[r].2 - sum[l].2 > 0 {
                continue;
            }
            answer = answer.min(sum[r].0 - sum[l].0);
        }
    }
    for j in 0..w {
        let mut sum = vec![(0, 0, 0); h + 1];
        for i in 0..h {
            sum[i + 1] = sum[i];
            match s[i][j] {
                '.' => {
                    sum[i + 1].0 += 1;
                }
                'o' => {
                    sum[i + 1].1 += 1;
                }
                'x' => {
                    sum[i + 1].2 += 1;
                }
                _ => unreachable!(),
            }
        }
        for l in 0..h {
            let r = l + k;
            if r > h {
                break;
            }
            if sum[r].2 - sum[l].2 > 0 {
                continue;
            }
            answer = answer.min(sum[r].0 - sum[l].0);
        }
    }
    println!("{}", if answer < std::i32::MAX { answer } else { -1 });
}
