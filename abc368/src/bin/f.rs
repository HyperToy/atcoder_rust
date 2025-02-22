use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }
    let mut sum = 0;
    for x in a.iter().map(|&x| prime_factorize(x)) {
        sum ^= x;
    }
    println!("{}", if sum != 0 { "Anna" } else { "Bruno" });
}

fn prime_factorize(mut n: u32) -> u32 {
    let mut res = 0;
    for a in 2.. {
        if a * a > n {
            break;
        }
        if n % a != 0 {
            continue;
        }
        let mut ex = 0;
        while n % a == 0 {
            ex += 1;
            n /= a;
        }
        res += ex;
    }
    if n != 1 {
        res += 1;
    };
    res
}
