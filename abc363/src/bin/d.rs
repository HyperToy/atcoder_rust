use proconio::input;

fn main() {
    input! {
        n: u64,
    }
    let n = n - 1; // 0 を 0番目とする
    println!("{}", palindromic(1, n));
}
fn palindromic(dig: u32, pos: u64) -> String {
    if pos == 0 {
        return String::from("0");
    }
    let half = (dig + 1) / 2;
    let count = 9 * 10u64.pow(half - 1);
    if 1 <= pos && pos <= count {
        let head = 10u64.pow(half - 1) + pos - 1;
        let s_head = head.to_string();
        let mut s_tail = s_head.chars().rev().collect::<String>();
        if dig < half * 2 {
            s_tail.remove(0);
        }
        return format!("{}{}", s_head, s_tail);
    }
    palindromic(dig + 1, pos - count)
}
