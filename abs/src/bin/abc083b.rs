use proconio::input;

fn main() {
    input! {
        n: u32,
        a: u32,
        b: u32,
    }
    let mut ans: u32 = 0;
    for i in 1..=n {
        let mut x: u32 = i;
        let mut cnt: u32 = 0;
        while x > 0 {
            cnt += x % 10;
            x /= 10;
        }
        if a <= cnt && cnt <= b {
            ans += i;
        }
    }
    println!("{}", ans);
}
