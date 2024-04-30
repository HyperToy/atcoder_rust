use proconio::input;

fn main() {
    input! {
        a: [i32; 9],
        b: [i32; 8],
    }
    println!("{}", a.iter().sum::<i32>() - b.iter().sum::<i32>() + 1);
}
