use proconio::input;

fn main() {
    input! {
        n: u64,
    }
    let mut answer = 1;
    for x in 2.. {
        let num = x * x * x;
        if num > n {
            break;
        }
        if is_kaibun(num) {
            answer = num;
        }
    }
    println!("{}", answer);
}

fn is_kaibun(x: u64) -> bool {
    let v = digits(x);
    let l = v.len();
    for i in 0..l / 2 {
        if v[i] != v[l - 1 - i] {
            return false;
        }
    }
    true
}

fn digits(mut x: u64) -> Vec<u64> {
    let mut res = Vec::new();
    while x > 0 {
        res.push(x % 10);
        x /= 10;
    }
    res.reverse();
    res
}
