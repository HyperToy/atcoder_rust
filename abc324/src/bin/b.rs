use proconio::input;

// refactor
fn main() {
    input! {
        n: i64,
    }
    for x in 0..60 {
        let num = 2i64.pow(x);
        if num > n {
            break;
        }
        for y in 0..38 {
            let num = num * 3i64.pow(y);
            if num == n {
                println!("Yes");
                return;
            }
            if num > n {
                break;
            }
        }
    }
    println!("No");
}
