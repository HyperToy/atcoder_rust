use proconio::input;

fn main() {
    input! {
        s: (char, char, char),
    }
    println!(
        "{}",
        match s {
            ('<', _, '<') => "B",
            ('>', _, '>') => "B",
            ('<', '>', _) => "A",
            ('>', '<', _) => "A",
            (_, '<', '>') => "C",
            (_, '>', '<') => "C",
            _ => unreachable!(),
        }
    )
}
