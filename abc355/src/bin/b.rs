use proconio::input;

fn main() {
    input! {
        n: usize, m: usize,
        a: [i32; n],
        b: [i32; m],
    }
    let mut c = vec![(0, 0); n + m];
    for i in 0..n {
        c[i] = (a[i], 0);
    }
    for j in 0..m {
        c[n + j] = (b[j], 1);
    }
    c.sort();
    let mut found = false;
    for i in 1..n + m {
        if c[i - 1].1 == 0 && c[i].1 == 0 {
            found = true;
        }
    }
    println!("{}", if found { "Yes" } else { "No" });
}
