use proconio::input;

// todo: refactor
fn main() {
    input! {
        n: usize, k: usize,
        cv: [(i32, i64); n],
    }
    let mut dp = vec![Top2::default(); k + 1];
    dp[0] = Top2::new(D::new(0, -1), D::default(-2));

    for i in 0..n {
        let mut now_dp = vec![Top2::default(); k + 1];
        for j in 0..=k {
            // 取り除く
            if j < k {
                now_dp[j + 1] = now_dp[j + 1].add2(dp[j]);
            }
            // 使う
            let max_value = dp[j].get(cv[i].0);
            let d = D::new(max_value + cv[i].1, cv[i].0);
            now_dp[j] = now_dp[j].add(d);
        }
        dp = now_dp;
    }
    let answer = dp[k].a.value;
    println!("{}", if answer < 0 { -1 } else { answer });
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct D {
    value: i64,
    color: i32,
}
impl D {
    fn default(color: i32) -> Self {
        Self {
            value: std::i64::MIN,
            color,
        }
    }
    fn new(value: i64, color: i32) -> Self {
        Self { value, color }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Top2 {
    a: D,
    b: D,
}
impl Top2 {
    fn default() -> Self {
        Self {
            a: D::new(std::i64::MIN + 1, -1),
            b: D::new(std::i64::MIN, -2),
        }
    }
    fn new(a: D, b: D) -> Self {
        Self { a, b }
    }

    // top2 に color,value をマージする
    fn add(&self, d: D) -> Self {
        let a = self.a;
        let b = self.b;
        let mut res = Self::new(a, b);
        if a.value < d.value {
            res.a = d;
            res.b = a;
        } else if b.value < d.value {
            res.b = d;
        }
        if res.a.color == res.b.color {
            res.b = b;
        }
        res
    }
    fn add2(&self, x: Top2) -> Self {
        self.add(x.a).add(x.b)
    }
    fn get(&self, color: i32) -> i64 {
        if self.a.color == color {
            self.b.value
        } else {
            self.a.value
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{Top2, D};

    #[test]
    fn check() {
        let a = D::new(200, 1);
        let b = D::new(100, 2);
        let c = D::new(300, 3);
        let d = D::new(150, 3);
        let e = D::new(300, 1);
        let f = D::new(150, 2);
        let t = Top2::new(a, b);
        let u = Top2::new(c, d);
        assert_eq!(t.add(c), Top2::new(c, a));
        assert_eq!(t.add(d), Top2::new(a, d));
        assert_eq!(t.add(e), Top2::new(e, b));
        assert_eq!(t.add(f), Top2::new(a, f));

        assert_eq!(t.get(1), 100);
        assert_eq!(t.get(2), 200);
        assert_eq!(t.add2(u), Top2::new(c, a));
    }
}
