use proconio::input;
type Cube = (i32, i32, i32);

fn main() {
    input! {
        v: (i32, i32, i32),
    }
    if v.0 + v.1 * 2 + v.2 * 3 != 3 * 7i32.pow(3) {
        println!("No");
        return;
    }

    let mut answer = None;
    let cube1 = (0, 0, 0);
    'outer: for a2 in 0..=7 {
        for b2 in 0..=7 {
            for c2 in 0..=7 {
                let cube2 = (a2, b2, c2);
                for a3 in -7..=7 {
                    for b3 in -7..=7 {
                        for c3 in -7..=7 {
                            let cube3 = (a3, b3, c3);
                            if calc((cube1, cube2, cube3)) == v {
                                answer = Some(format!(
                                    "0 0 0 {} {} {} {} {} {}",
                                    a2, b2, c2, a3, b3, c3
                                ));
                                break 'outer;
                            }
                        }
                    }
                }
            }
        }
    }
    println!(
        "{}",
        match answer {
            Some(s) => "Yes ".to_string() + s.as_str(),
            None => "No".to_string(),
        }
    );
}

fn calc(c: (Cube, Cube, Cube)) -> (i32, i32, i32) {
    let v3 = v3(c);
    let v2 = v2((c.0, c.1)) + v2((c.1, c.2)) + v2((c.2, c.0)) - v3 * 3;
    let v1 = 3 * 7i32.pow(3) - v2 * 2 - v3 * 3;
    (v1, v2, v3)
}

fn v3(c: (Cube, Cube, Cube)) -> i32 {
    ((c.0 .0.min(c.1 .0).min(c.2 .0) + 7) - (c.0 .0.max(c.1 .0).max(c.2 .0))).max(0)
        * ((c.0 .1.min(c.1 .1).min(c.2 .1) + 7) - (c.0 .1.max(c.1 .1).max(c.2 .1))).max(0)
        * ((c.0 .2.min(c.1 .2).min(c.2 .2) + 7) - (c.0 .2.max(c.1 .2).max(c.2 .2))).max(0)
}
fn v2(c: (Cube, Cube)) -> i32 {
    ((c.0 .0.min(c.1 .0) + 7) - (c.0 .0.max(c.1 .0))).max(0)
        * ((c.0 .1.min(c.1 .1) + 7) - (c.0 .1.max(c.1 .1))).max(0)
        * ((c.0 .2.min(c.1 .2) + 7) - (c.0 .2.max(c.1 .2))).max(0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check() {
        let c = ((0, 0, 0), (0, 6, 0), (6, 0, 0));
        assert_eq!(v3(c), 7);
        assert_eq!(calc(c), (840, 84, 7));
    }
}
