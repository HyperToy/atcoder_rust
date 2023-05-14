use proconio::*;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }

    let mut ans = vec![0; n * 2 + 1];
    ans[0] = 0;
    for i in 0..n {
        ans[i * 2 + 1] = ans[a[i] as usize - 1] + 1;
        ans[i * 2 + 2] = ans[a[i] as usize - 1] + 1;
    }

    for i in 0..n * 2 + 1 {
        println!("{}", ans[i]);
    }
}
