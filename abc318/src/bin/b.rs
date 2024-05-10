use proconio::input;

fn main() {
    input! {
        n: usize,
        abcd: [(usize, usize, usize, usize); n],
    }
    let h = 1 + *abcd.iter().map(|(a, b, _, _)| a.max(b)).max().unwrap();
    let w = 1 + *abcd.iter().map(|(_, _, c, d)| c.max(d)).max().unwrap();
    let mut coor = vec![vec![0; w]; h];
    for (a, b, c, d) in abcd {
        coor[a][c] += 1;
        coor[a][d] -= 1;
        coor[b][c] -= 1;
        coor[b][d] += 1;
    }
    for i in 0..h {
        for j in 0..w {
            coor[i][j] += if i > 0 { coor[i - 1][j] } else { 0 };
            coor[i][j] += if j > 0 { coor[i][j - 1] } else { 0 };
            coor[i][j] -= if i * j > 0 { coor[i - 1][j - 1] } else { 0 };
        }
    }
    println!("{}", coor.iter().flatten().filter(|x| **x > 0).count());
}
