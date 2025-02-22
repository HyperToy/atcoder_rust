use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n * 2],
    }
    let mut pos = vec![Vec::new(); n];
    for i in 0..n * 2 {
        pos[a[i]].push(i);
    }
    println!("{}", pos.into_iter().filter(|v| v[0] + 2 == v[1]).count());
}
