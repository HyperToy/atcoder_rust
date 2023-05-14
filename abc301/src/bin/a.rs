use proconio::*;
use std::cmp::Ordering;

fn main() {
    input! {
        n: usize,
        s: String,
    }
    let mut t = 0;
    for c in s.chars() {
        match c {
            'T' => t += 1,
            _ => (),
        }
    }
    let a = n - t;
    println!(
        "{}",
        match t.cmp(&a) {
            Ordering::Greater => 'T',
            Ordering::Less => 'A',
            Ordering::Equal => match s.chars().nth(s.len() - 1).unwrap() {
                'A' => 'T',
                'T' => 'A',
                _ => unreachable!(),
            },
        }
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ordering() {
        assert_eq!(Ordering::Less, 1.cmp(&2));
        assert_eq!(Ordering::Equal, 1.cmp(&1));
        assert_eq!(Ordering::Greater, 2.cmp(&1));
    }
    #[test]
    fn last_char() {
        let s = String::from("hello");
        assert_eq!('o', s.chars().nth(s.len() - 1).unwrap());
    }
}
