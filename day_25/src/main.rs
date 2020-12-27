const HANDSHAKE_SUBJECT: u64 = 7;
const SECRET: u64 = 20201227;

use std::ops::Range;

fn main() {
    // Input:
    let card_pub_key = 2069194;
    let door_pub_key = 16426071;

    let part_1 = part_1(card_pub_key, door_pub_key);
    assert_eq!(part_1, 11576351);
    println!("Part 1: {}", part_1);
}

fn part_1(card_pub_key: u64, door_pub_key: u64) -> u64 {
    find_encryption_key(card_pub_key, door_pub_key)
}

fn find_encryption_key(card_pub_key: u64, door_pub_key: u64) -> u64 {
    let door_loop_size = find_loop_size(door_pub_key, 1);
    transform(card_pub_key, door_loop_size)
}

fn find_loop_size(value: u64, starting_loop_size: u64) -> u64 {
    let mut last_value = 1;
    for loop_size in starting_loop_size.. {
        last_value =
            transform_with_initial_value(HANDSHAKE_SUBJECT, (loop_size - 1)..loop_size, last_value);
        if last_value == value {
            return loop_size;
        }
    }
    unreachable!();
}

// The handshake used by the card and the door involves an operation that transforms a subject
// number. To transform a subject number, start with the value 1. Then, a number of times called
// the loop size, perform the following steps:
//
//     Set the value to itself multiplied by the subject number.
//     Set the value to the remainder after dividing the value by 20201227.
//
// The card always uses a specific, secret loop size when it transforms a subject number. The door
// always uses a different, secret loop size.
fn transform(subject: u64, loop_size: u64) -> u64 {
    transform_with_initial_value(subject, 0..loop_size, 1)
}

fn transform_with_initial_value(subject: u64, loop_size: Range<u64>, mut value: u64) -> u64 {
    for _ in loop_size {
        value = (value * subject) % SECRET;
    }
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn readme_example() {
        // For example, suppose you know that the card's public key is 5764801. With a little trial
        // and error, you can work out that the card's loop size must be 8, because transforming
        // the initial subject number of 7 with a loop size of 8 produces 5764801.
        let card_pub_key = 5764801;
        let card_loop_size = 8;
        assert_eq!(transform(HANDSHAKE_SUBJECT, card_loop_size), card_pub_key);
        assert_eq!(find_loop_size(card_pub_key, 1), card_loop_size);

        // Then, suppose you know that the door's public key is 17807724. By the same process, you
        // can determine that the door's loop size is 11, because transforming the initial subject
        // number of 7 with a loop size of 11 produces 17807724.
        let door_pub_key = 17807724;
        let door_loop_size = 11;
        assert_eq!(transform(HANDSHAKE_SUBJECT, door_loop_size), door_pub_key);
        assert_eq!(find_loop_size(door_pub_key, 1), door_loop_size);

        // At this point, you can use either device's loop size with the other device's public key
        // to calculate the encryption key. Transforming the subject number of 17807724 (the door's
        // public key) with a loop size of 8 (the card's loop size) produces the encryption key,
        // 14897079. (Transforming the subject number of 5764801 (the card's public key) with a
        // loop size of 11 (the door's loop size) produces the same encryption key: 14897079.)
        let encryption_key = 14897079;
        assert_eq!(transform(door_pub_key, card_loop_size), encryption_key);
        assert_eq!(transform(card_pub_key, door_loop_size), encryption_key);

        // Using the above information, without the secret loop sizes, find the encryption_key.
        assert_eq!(
            find_encryption_key(card_pub_key, door_pub_key),
            encryption_key
        );
    }
}
