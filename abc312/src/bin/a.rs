use proconio::*;

fn main() {
    input! {
        s: String,
    }
    println!(
        "{}",
        match s.as_str() {
            "ACE" | "BDF" | "CEG" | "DFA" | "EGB" | "FAC" | "GBD" => "Yes",
            _ => "No",
        }
    );
}
