use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    let mut v = Vec::new();
    for i in 0..n {
        v.push(a[i]);
        while v.len() >= 2 && v[v.len() - 2] == v[v.len() - 1] {
            let x = v[v.len() - 1];
            v.pop();
            v.pop();
            v.push(x + 1);
        }
    }
    println!("{}", v.len());
}
