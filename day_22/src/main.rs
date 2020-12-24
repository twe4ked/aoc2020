const PLAYER_1: [u8; 25] = [
    10, 21, 37, 2, 47, 13, 6, 29, 9, 3, 4, 48, 46, 25, 44, 41, 23, 20, 24, 12, 45, 43, 5, 27, 50,
];

const PLAYER_2: [u8; 25] = [
    39, 42, 31, 36, 7, 1, 49, 19, 40, 35, 8, 11, 18, 30, 14, 17, 15, 34, 26, 33, 32, 38, 28, 16, 22,
];

use std::collections::VecDeque;

fn main() {
    let part_1 = part_1();
    assert_eq!(part_1, 115);
    println!("Part 1: {}", part_1);
}

fn part_1() -> usize {
    part_1_inner(
        VecDeque::from(PLAYER_1.to_vec()),
        VecDeque::from(PLAYER_2.to_vec()),
    )
}

fn part_1_inner(mut player_1: VecDeque<u8>, mut player_2: VecDeque<u8>) -> usize {
    let winner = loop {
        match (player_1.pop_front(), player_2.pop_front()) {
            (Some(player_1_card), Some(player_2_card)) => {
                if player_1_card > player_2_card {
                    // Player 1 wins
                    player_1.push_back(player_1_card);
                    player_1.push_back(player_2_card);
                } else {
                    // Player 2 wins
                    player_2.push_back(player_2_card);
                    player_2.push_back(player_1_card);
                }
            }
            (Some(player_1_card), None) => {
                // Put the card back
                player_1.push_front(player_1_card);
                break player_1;
            }
            (None, Some(player_2_card)) => {
                // Put the card back
                player_2.push_front(player_2_card);
                break player_2;
            }
            _ => unreachable!(),
        }
    };

    // Calculate score
    winner
        .iter()
        .rev()
        .enumerate()
        .map(|(i, card)| *card as usize * (i + 1))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn readme_example() {
        let player_1 = VecDeque::from(vec![9, 2, 6, 3, 1]);
        let player_2 = VecDeque::from(vec![5, 8, 4, 7, 10]);

        assert_eq!(part_1_inner(player_1, player_2), 306);
    }
}
