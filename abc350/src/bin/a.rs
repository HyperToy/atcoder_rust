use proconio::input;

fn main() {
    input! {
        s: String,
    }
    println!(
        "{}",
        if (1..316)
            .chain(317..350)
            .any(|i| format! {"ABC{:03}",i} == s)
        {
            "Yes"
        } else {
            "No"
        }
    );
}
