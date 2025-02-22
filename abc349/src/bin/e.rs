use itertools::Itertools;
use proconio::input;
use std::{cmp::Ordering, collections::HashMap, convert::TryInto};

fn main() {
    input! {
        a: [i64; 9],
    }
    let a = a.try_into().unwrap();
    let mut dp = HashMap::new();
    println!(
        "{}",
        match rec(&a, [None; 9], &mut dp, Player::Takahashi) {
            Player::Takahashi => "Takahashi",
            Player::Aoki => "Aoki",
        }
    );
}
fn rec(
    a: &[i64; 9],
    state: [Option<Player>; 9],
    dp: &mut HashMap<[Option<Player>; 9], Player>,
    player: Player,
) -> Player {
    if let Some(&winner) = dp.get(&state) {
        return winner;
    }
    if let Some(winner) = line(&state) {
        dp.insert(state, winner);
        return winner;
    }
    if state.iter().all(|&x| x.is_some()) {
        let takahashi = score(a, Player::Takahashi, &state);
        let aoki = score(a, Player::Aoki, &state);
        let winner = match takahashi.cmp(&aoki) {
            Ordering::Greater => Player::Takahashi,
            Ordering::Less => Player::Aoki,
            Ordering::Equal => unreachable!(),
        };
        dp.insert(state, winner);
        return winner;
    }
    let winner = if (0..9)
        .filter(|&i| state[i].is_none())
        .map(|i| {
            let mut next_state = state.clone();
            next_state[i] = Some(player);
            next_state
        })
        .map(|state| rec(a, state, dp, player.rev()))
        .any(|winner| winner == player)
    {
        player
    } else {
        player.rev()
    };
    dp.insert(state, winner);
    winner
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Player {
    Takahashi,
    Aoki,
}
impl Player {
    fn rev(&self) -> Self {
        match self {
            Self::Takahashi => Self::Aoki,
            Self::Aoki => Self::Takahashi,
        }
    }
}

fn score(a: &[i64; 9], p: Player, state: &[Option<Player>; 9]) -> i64 {
    state
        .iter()
        .enumerate()
        .filter_map(|(i, &x)| if x == Some(p) { Some(a[i]) } else { None })
        .sum::<i64>()
}
fn line(state: &[Option<Player>; 9]) -> Option<Player> {
    if line2(state, Player::Takahashi) {
        Some(Player::Takahashi)
    } else if line2(state, Player::Aoki) {
        Some(Player::Aoki)
    } else {
        None
    }
}
fn line2(state: &[Option<Player>; 9], player: Player) -> bool {
    (0..3)
        .map(|i| (0..3).all(|j| state[i * 3 + j] == Some(player)))
        .contains(&true)
        || (0..3)
            .map(|j| (0..3).all(|i| state[i * 3 + j] == Some(player)))
            .contains(&true)
        || (0..3)
            .zip(0..3)
            .all(|(i, j)| state[i * 3 + j] == Some(player))
        || (0..3)
            .zip((0..3).rev())
            .all(|(i, j)| state[i * 3 + j] == Some(player))
}
