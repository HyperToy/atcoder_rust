use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize, q: usize,
    }
    let mut pos = (0..n).collect::<Vec<_>>();
    // 箱i は今は s[i] と呼ばれている
    let mut s = (0..n).collect::<Vec<_>>();
    let mut si = (0..n).collect::<Vec<_>>();
    for _ in 0..q {
        input! { t: i32 }
        match t {
            1 => {
                input! { a: Usize1, b: Usize1 }
                pos[a] = si[b];
            }
            2 => {
                input! { a: Usize1, b: Usize1 }
                swap(&mut s, a, b);
                swap(&mut si, a, b);
            }
            3 => {
                input! { a: Usize1 }
                println!("{}", s[pos[a]] + 1);
            }
            _ => unreachable!(),
        }
    }
}
fn swap(v: &mut Vec<usize>, a: usize, b: usize) {
    let tmp = v[a];
    v[a] = b;
    v[b] = tmp;
}
