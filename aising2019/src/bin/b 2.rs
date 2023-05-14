use proconio::*;

fn main() {
    input! {
        n: usize,
        a: u32, b: u32,
        p: [u32; n],
    }
    let mut vec = vec![0; 3];
    for x in p {
        if x <= a {
            vec[0] += 1;
        } else if x <= b {
            vec[1] += 1;
        } else {
            vec[2] += 1;
        }
    }
    vec.sort();
    println!("{}", vec[0]);
}
