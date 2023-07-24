use proconio::{marker::Bytes, *};

fn main() {
    input! {
        n: usize,
        s: Bytes,
    }
    let mut count = vec![0; 3];
    for i in 0..n {
        count[(s[i] - b'A') as usize] += 1;
        if count.iter().all(|x| x > &0) {
            println!("{}", i + 1);
            break;
        }
    }
}
