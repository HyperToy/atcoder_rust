use proconio::input;

fn main() {
    input! {
        k: u64,
    }
    let mut answer = 0;
    for (p, e) in prime_factorize(k) {
        let mut min = p;
        let mut count = 1;
        while count < e {
            min += p;
            count += prime_factorize(min)
                .into_iter()
                .find(|&(n, _)| n == p)
                .unwrap()
                .1;
        }
        answer = answer.max(min);
    }
    println!("{}", answer)
}

fn prime_factorize(mut n: u64) -> Vec<(u64, u32)> {
    let mut res = Vec::new();
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
        res.push((a, ex));
    }
    if n != 1 {
        res.push((n, 1))
    };
    res
}
