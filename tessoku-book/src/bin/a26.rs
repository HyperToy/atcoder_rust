use proconio::*;

fn main() {
    input! {
        q: usize,
        xs: [i32; q],
    }
    for x in xs {
        println!("{}", if is_prime(x) { "Yes" } else { "No" });
    }
}

fn is_prime(x: i32) -> bool {
    let mut res = true;
    let mut i = 2;
    while i * i <= x {
        if x % i == 0 {
            res = false;
            break;
        }
        i += 1;
    }
    res
}
