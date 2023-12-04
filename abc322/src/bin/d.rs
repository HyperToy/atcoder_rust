use proconio::{input, marker::Chars};

fn main() {
    input! {
        ps: [[Chars; 4]; 3],
    }
    let ps = ps
        .into_iter()
        .map(|p| Polyomino::from(p))
        .collect::<Vec<_>>();
}

struct Polyomino {
    height: usize,
    width: usize,
    minos: Vec<(usize, usize)>,
}

impl Polyomino {
    fn from(p: Vec<Vec<char>>) -> Self {
        let mut top = p.len();
        let mut bottom = 0;
        for i in 0..p.len() {
            if p[i].iter().any(|c| c == &'#') {
                top = top.min(i);
                bottom = bottom.max(i);
            }
        }
        let mut left = 0;
        let mut right = p[0].len() - 1;
        for j in 0..p[0].len() {
            for i in 0..p.len() {
                if p[i][j] == '#' {
                    left = left.min(j);
                    right = right.max(j);
                    break;
                }
            }
        }
        let height = bottom - top + 1;
        let width = right - left + 1;
        let mut minos = vec![];
        for i in 0..height {
            for j in 0..width {
                if p[top + i][left + j] == '#' {
                    minos.push((i, j));
                }
            }
        }
        Polyomino {
            height,
            width,
            minos,
        }
    }
}
