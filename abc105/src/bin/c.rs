use proconio::*;

fn main() {
    input! {
        n: i32,
    }
    println!("{}", convert(n));
}

fn convert(mut n: i32) -> String {
    let mut res = String::new();
    while n != 0 {
        let (q, r) = div_mod(n);
        res.insert_str(0, &r.to_string());
        n = q;
    }
    if res.len() == 0 {
        res.push('0');
    }
    res
}

// n = (-2) * q + r (r = 0, 1)
fn div_mod(n: i32) -> (i32, i32) {
    let r = if n > 0 { n } else { -n } % 2;
    let q = (n - r) / -2;
    (q, r)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_mod() {
        assert_eq!((-2, 0), div_mod(4));
        assert_eq!((-2, 1), div_mod(5));
        assert_eq!((2, 0), div_mod(-4));
        assert_eq!((3, 1), div_mod(-5));
    }
}
