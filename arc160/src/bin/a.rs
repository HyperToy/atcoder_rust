use proconio::*;

fn main() {
    input! {
        n: usize,
        mut k: usize,
        mut a: [u32; n],
    }
    k -= 1;
    let mut inv = 0;
    for i in 0..n {
        for j in 0..i {
            if a[j] > a[i] {
                inv += 1;
            }
        }
    }
    if k < inv || inv + n <= k {
        let (l, r) = rec(&a, 0, k);
        a[l..r].reverse();
    }
    for i in 0..n {
        print!("{} ", a[i]);
    }
    println!();
}

fn rec(a: &Vec<u32>, offset: usize, k: usize) -> (usize, usize) {
    let n = a.len() - offset;
    let middle = n * (n - 1) / 2 + 1;

    let mut vec = Vec::new();
    let mut smaller = 0;
    for i in offset + 1..a.len() {
        vec.push((a[i], i));
        if a[i] < a[offset] {
            smaller += 1;
        }
    }
    vec.sort();
    if k < smaller {
        return (offset, (vec[k].1) + 1);
    }
    if smaller + middle <= k {
        return (offset, (vec[k - middle].1) + 1);
    }
    rec(&a, offset + 1, k - smaller)
}
