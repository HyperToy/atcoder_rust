use proconio::{input, marker::Chars};

// upsolve
fn main() {
    input! {
        (ha, _wa): (usize, usize),
        a: [Chars; ha],
        (hb, _wb): (usize, usize),
        b: [Chars; hb],
        (hx, _wx): (usize, usize),
        x: [Chars; hx],
    }
    let (ha, wa, a) = cut(a);
    let (hb, wb, b) = cut(b);
    let (hx, wx, x) = cut(x);
    if ha > hx || wa > wx || hb > hx || wb > wx {
        println!("No");
        return;
    }
    let mut ok = false;
    for ia in 0..hx - ha + 1 {
        for ja in 0..wx - wa + 1 {
            for ib in 0..hx - hb + 1 {
                for jb in 0..wx - wb + 1 {
                    let mut now = vec![vec!['.'; wx]; hx];
                    for i in 0..ha {
                        for j in 0..wa {
                            if a[i][j] == '#' {
                                now[ia + i][ja + j] = '#';
                            }
                        }
                    }
                    for i in 0..hb {
                        for j in 0..wb {
                            if b[i][j] == '#' {
                                now[ib + i][jb + j] = '#';
                            }
                        }
                    }
                    if now == x {
                        ok = true;
                    }
                }
            }
        }
    }
    println!("{}", if ok { "Yes" } else { "No" });
}

fn cut(a: Vec<Vec<char>>) -> (usize, usize, Vec<Vec<char>>) {
    let h = a.len();
    let w = a[0].len();
    let mut top = h;
    let mut bottom = 0;
    let mut left = w;
    let mut right = 0;
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == '#' {
                top = top.min(i);
                bottom = bottom.max(i + 1);
                left = left.min(j);
                right = right.max(j + 1);
            }
        }
    }
    let h = bottom - top;
    let w = right - left;
    let mut res = Vec::new();
    for i in 0..h {
        let mut row = Vec::new();
        for j in 0..w {
            row.push(a[top + i][left + j]);
        }
        res.push(row);
    }
    (h, w, res)
}
