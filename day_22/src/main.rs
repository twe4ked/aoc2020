const PLAYER_1: [u8; 25] = [
    10, 21, 37, 2, 47, 13, 6, 29, 9, 3, 4, 48, 46, 25, 44, 41, 23, 20, 24, 12, 45, 43, 5, 27, 50,
];

const PLAYER_2: [u8; 25] = [
    39, 42, 31, 36, 7, 1, 49, 19, 40, 35, 8, 11, 18, 30, 14, 17, 15, 34, 26, 33, 32, 38, 28, 16, 22,
];

use std::collections::VecDeque;

fn main() {
    let part_1 = part_1();
    assert_eq!(part_1, 33_631);
    println!("Part 1: {}", part_1);

    let part_2 = part_2();
    assert_eq!(part_2, 33_469);
    println!("Part 2: {}", part_2);
}

fn part_1() -> usize {
    part_1_inner(
        VecDeque::from(PLAYER_1.to_vec()),
        VecDeque::from(PLAYER_2.to_vec()),
    )
}

fn part_2() -> usize {
    part_2_inner(
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

    calculate_score(winner)
}

fn part_2_inner(player_1: VecDeque<u8>, player_2: VecDeque<u8>) -> usize {
    let (_, winner) = part_2_inner_game(player_1, player_2, 1);

    calculate_score(winner)
}

enum Winner {
    Player1,
    Player2,
}

macro_rules! debug_print {
    ($($arg:tt)*) => {{
        if option_env!("DEBUG").map(|_| true).unwrap_or(false) {
            println!($($arg)*)
        }
    }}
}

fn part_2_inner_game(
    mut player_1: VecDeque<u8>,
    mut player_2: VecDeque<u8>,
    game_number: usize,
) -> (Winner, VecDeque<u8>) {
    debug_print!("=== Game {} ===\n", game_number);

    let mut previous_rounds = Vec::new();
    let mut round_number = 1;

    loop {
        debug_print!("-- Round {} (Game {}) --", round_number, game_number);
        debug_print!("Player 1's deck: {:?}", &player_1);
        debug_print!("Player 2's deck: {:?}", &player_2);

        // Before either player deals a card, if there was a previous round in this game that had
        // exactly the same cards in the same order in the same players' decks, the game instantly
        // ends in a win for player 1. Previous rounds from other games are not considered. (This
        // prevents infinite games of Recursive Combat, which everyone agrees is a bad idea.)
        if previous_rounds.contains(&player_1) || previous_rounds.contains(&player_2) {
            return (Winner::Player1, player_1);
        }
        previous_rounds.push(player_1.clone());
        previous_rounds.push(player_2.clone());

        // Otherwise, this round's cards must be in a new configuration; the players begin the
        // round by each drawing the top card of their deck as normal.
        match (player_1.pop_front(), player_2.pop_front()) {
            (Some(player_1_card), Some(player_2_card)) => {
                debug_print!("Player 1 plays: {}", player_1_card);
                debug_print!("Player 2 plays: {}", player_2_card);

                if player_1.len() >= player_1_card as usize
                    && player_2.len() >= player_2_card as usize
                {
                    debug_print!("Playing a sub-game to determine the winner...");
                    // If both players have at least as many cards remaining in their deck as the
                    // value of the card they just drew, the winner of the round is determined by
                    // playing a new game of Recursive Combat.
                    //
                    // We have enough cards to recurse!
                    match part_2_inner_game(
                        player_1.iter().copied().take(player_1_card as _).collect(),
                        player_2.iter().copied().take(player_2_card as _).collect(),
                        game_number + 1,
                    ) {
                        (Winner::Player1, _) => {
                            debug_print!(
                                "Player 1 wins round {} of game {}!\n",
                                round_number,
                                game_number
                            );
                            player_1.push_back(player_1_card);
                            player_1.push_back(player_2_card);
                        }
                        (Winner::Player2, _) => {
                            debug_print!(
                                "Player 2 wins round {} of game {}!\n",
                                round_number,
                                game_number
                            );
                            player_2.push_back(player_2_card);
                            player_2.push_back(player_1_card);
                        }
                    }
                } else {
                    // Otherwise, at least one player must not have enough cards left in their deck
                    // to recurse; the winner of the round is the player with the higher-value
                    // card.
                    if player_1_card > player_2_card {
                        // Player 1 wins
                        debug_print!(
                            "Player 1 wins round {} of game {}!\n",
                            round_number,
                            game_number
                        );
                        player_1.push_back(player_1_card);
                        player_1.push_back(player_2_card);
                    } else {
                        // Player 2 wins
                        debug_print!(
                            "Player 2 wins round {} of game {}!\n",
                            round_number,
                            game_number
                        );
                        player_2.push_back(player_2_card);
                        player_2.push_back(player_1_card);
                    }
                }
            }
            (Some(player_1_card), None) => {
                // Put the card back
                player_1.push_front(player_1_card);
                return (Winner::Player1, player_1);
            }
            (None, Some(player_2_card)) => {
                // Put the card back
                player_2.push_front(player_2_card);
                return (Winner::Player2, player_2);
            }
            _ => unreachable!(),
        }

        round_number += 1;
    }
}

fn calculate_score(deck: VecDeque<u8>) -> usize {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(i, card)| *card as usize * (i + 1))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn readme_example_part_1() {
        let player_1 = VecDeque::from(vec![9, 2, 6, 3, 1]);
        let player_2 = VecDeque::from(vec![5, 8, 4, 7, 10]);

        assert_eq!(part_1_inner(player_1, player_2), 306);
    }

    #[test]
    fn readme_example_part_2() {
        let player_1 = VecDeque::from(vec![9, 2, 6, 3, 1]);
        let player_2 = VecDeque::from(vec![5, 8, 4, 7, 10]);

        assert_eq!(part_2_inner(player_1, player_2), 291);
    }

    #[test]
    fn readme_example_recursive() {
        let player_1 = VecDeque::from(vec![43, 19]);
        let player_2 = VecDeque::from(vec![2, 29, 14]);

        part_2_inner(player_1, player_2);

        // It does not recurse forever!
        assert!(true);
    }
}
