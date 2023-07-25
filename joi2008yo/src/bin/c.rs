use proconio::{marker::Usize1, *};

fn main() {
    input! {
        n: usize,
        taros_cards: [Usize1; n],
    }
    let mut is_taros_card = vec![false; 2 * n];
    for i in 0..n {
        is_taros_card[taros_cards[i]] = true;
    }
    let mut is_drawn = vec![false; 2 * n];
    let mut is_taros_turn = true;
    let mut top = -1;
    let mut taros_rem = n;
    let mut hanakos_rem = n;
    loop {
        for i in 0..2 * n {
            if !(is_taros_card[i] ^ is_taros_turn) && !is_drawn[i] && i as isize > top {
                top = i as isize;
                is_drawn[i] = true;
                is_taros_turn ^= true;
                if is_taros_card[i] {
                    taros_rem -= 1;
                    if taros_rem == 0 {
                        break;
                    }
                } else {
                    hanakos_rem -= 1;
                    if hanakos_rem == 0 {
                        break;
                    }
                }
            }
        }
        if taros_rem == 0 || hanakos_rem == 0 {
            break;
        }
        top = -1;
        is_taros_turn ^= true;
    }
    println!("{}", hanakos_rem);
    println!("{}", taros_rem);
}
