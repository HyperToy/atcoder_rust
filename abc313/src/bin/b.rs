use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize, m: usize,
        ab: [(Usize1, Usize1); m],
    }
    const MAX: i32 = std::i32::MAX / 2;
    let mut d = vec![vec![MAX; n]; n];
    for i in 0..n {
        d[i][i] = 0;
    }
    for (a, b) in ab {
        d[a][b] = 1;
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                d[i][j] = d[i][j].min(d[i][k] + d[k][j]);
            }
        }
    }
    let mut answer = -1;
    for i in 0..n {
        if d[i].iter().all(|&x| x < MAX) {
            answer = i as isize + 1;
        }
    }
    println!("{}", answer);
}
