// --- Day 23: Crab Cups ---
//
// The small crab challenges you to a game! The crab is going to mix up some cups, and you have to
// predict where they'll end up.
//
// The cups will be arranged in a circle and labeled clockwise (your puzzle input). For example, if
// your labeling were 32415, there would be five cups in the circle; going clockwise around the
// circle from the first cup, the cups would be labeled 3, 2, 4, 1, 5, and then back to 3 again.
//
// Before the crab starts, it will designate the first cup in your list as the current cup. The
// crab is then going to do 100 moves.
//
// Each move, the crab does the following actions:
//
//     The crab picks up the three cups that are immediately clockwise of the current cup. They are
//      removed from the circle; cup spacing is adjusted as necessary to maintain the circle.
//     The crab selects a destination cup: the cup with a label equal to the current cup's label
//      minus one. If this would select one of the cups that was just picked up, the crab will keep
//      subtracting one until it finds a cup that wasn't just picked up. If at any point in this
//      process the value goes below the lowest value on any cup's label, it wraps around to the
//      highest value on any cup's label instead.
//     The crab places the cups it just picked up so that they are immediately clockwise of the
//      destination cup. They keep the same order as when they were picked up.
//     The crab selects a new current cup: the cup which is immediately clockwise of the current
//      cup.
//
// For example, suppose your cup labeling were 389125467. If the crab were to do merely 10 moves,
// the following changes would occur:
//
// -- move 1 --
// cups: (3) 8  9  1  2  5  4  6  7
// pick up: 8, 9, 1
// destination: 2
//
// -- move 2 --
// cups:  3 (2) 8  9  1  5  4  6  7
// pick up: 8, 9, 1
// destination: 7
//
// -- move 3 --
// cups:  3  2 (5) 4  6  7  8  9  1
// pick up: 4, 6, 7
// destination: 3
//
// -- move 4 --
// cups:  7  2  5 (8) 9  1  3  4  6
// pick up: 9, 1, 3
// destination: 7
//
// -- move 5 --
// cups:  3  2  5  8 (4) 6  7  9  1
// pick up: 6, 7, 9
// destination: 3
//
// -- move 6 --
// cups:  9  2  5  8  4 (1) 3  6  7
// pick up: 3, 6, 7
// destination: 9
//
// -- move 7 --
// cups:  7  2  5  8  4  1 (9) 3  6
// pick up: 3, 6, 7
// destination: 8
//
// -- move 8 --
// cups:  8  3  6  7  4  1  9 (2) 5
// pick up: 5, 8, 3
// destination: 1
//
// -- move 9 --
// cups:  7  4  1  5  8  3  9  2 (6)
// pick up: 7, 4, 1
// destination: 5
//
// -- move 10 --
// cups: (5) 7  4  1  8  3  9  2  6
// pick up: 7, 4, 1
// destination: 3
//
// -- final --
// cups:  5 (8) 3  7  4  1  9  2  6
//
// In the above example, the cups' values are the labels as they appear moving clockwise around the
// circle; the current cup is marked with ( ).
//
// After the crab is done, what order will the cups be in? Starting after the cup labeled 1,
// collect the other cups' labels clockwise into a single string with no extra characters; each
// number except 1 should appear exactly once. In the above example, after 10 moves, the cups
// clockwise from 1 are labeled 9, 2, 6, 5, and so on, producing 92658374. If the crab were to
// complete all 100 moves, the order after cup 1 would be 67384529.
//
// Using your labeling, simulate 100 moves. What are the labels on the cups after cup 1?

use std::fmt::Write;

fn main() {
    let part_1 = part_1(vec![7, 8, 4, 2, 3, 5, 9, 1, 6]);
    // assert_eq!(part_1, 99);
    println!("Part 1: {}", part_1);
}

type Cup = u8;
type Cups = Vec<Cup>;
type CupsSlice = [Cup];

fn part_1(input: Cups) -> u64 {
    let mut cups = input;
    let mut current_cup_index = 0;
    for move_number in 1..=100 {
        let (new_cups, new_index) = make_move(cups, current_cup_index, move_number);
        cups = new_cups;
        current_cup_index = new_index;
        assert_ne!(67384529, cups_to_u64(&cups));
    }

    println!("{:?}", &cups);

    cups_to_u64(&cups)
}

fn cups_to_u64(cups: &CupsSlice) -> u64 {
    let one_idx = cups.iter().position(|c| c == &1).unwrap();
    let inc = |n| wrap_inc(n, 0, 8);
    let mut result = String::new();
    let mut i = inc(one_idx);
    for _ in 0..8 {
        write!(&mut result, "{}", cups[i]).unwrap();
        i = inc(i);
    }
    result.parse().unwrap()
}

fn wrap_inc(n: usize, min: usize, max: usize) -> usize {
    wrap_add(n, 1, min, max)
}

fn wrap_dec(n: usize, min: usize, max: usize) -> usize {
    if n == min {
        max
    } else {
        n - 1
    }
}

fn wrap_add(a: usize, b: usize, min: usize, max: usize) -> usize {
    if a + b > max {
        min
    } else {
        a + b
    }
}

fn print_cups(cups: &CupsSlice, current_cup_label: Cup) {
    print!("cups: ");
    for cup in cups {
        if cup == &current_cup_label {
            print!("({})", cup);
        } else {
            print!("{}", cup);
        }

        print!(" ");
    }
    println!();
}

fn make_move(mut cups: Cups, current_cup_index: usize, move_number: usize) -> (Cups, usize) {
    println!("-- move {} --", move_number);

    let current_cup_label = cups[current_cup_index];

    print_cups(&cups, current_cup_label);

    // The crab picks up the three cups that are immediately clockwise of the current cup. They are
    // removed from the circle; cup spacing is adjusted as necessary to maintain the circle.
    let picked_up = pickup_cups(&mut cups, current_cup_index);
    println!("pick up: {:?}", picked_up);

    // The crab selects a destination cup: the cup with a label equal to the current cup's label
    // minus one. If this would select one of the cups that was just picked up, the crab will keep
    // subtracting one until it finds a cup that wasn't just picked up. If at any point in this
    // process the value goes below the lowest value on any cup's label, it wraps around to the
    // highest value on any cup's label instead.
    let (destination_cup_label, destination_cup_idx) =
        select_destination_cup(&cups, current_cup_label);
    println!(
        "destination: {} (idx: {})",
        destination_cup_label, destination_cup_idx
    );
    println!();

    // The crab places the cups it just picked up so that they are immediately clockwise of the
    // destination cup. They keep the same order as when they were picked up.
    place_cups(&mut cups, picked_up, destination_cup_idx);

    // The crab selects a new current cup: the cup which is immediately clockwise of the current
    // cup.
    let new_current_cup_idx = select_new_current_cup_idx(&cups, current_cup_label);

    (cups, new_current_cup_idx)
}

fn place_cups(cups: &mut Cups, picked_up: Cups, destination_cup_idx: usize) {
    assert_eq!(cups.len(), 6);

    let mut idx = wrap_inc(destination_cup_idx, 0, cups.len() - 1);
    cups.insert(idx, picked_up[0]);
    assert_eq!(cups.len(), 7);

    idx = wrap_inc(idx, 0, cups.len() - 1);
    cups.insert(idx, picked_up[1]);
    assert_eq!(cups.len(), 8);

    idx = wrap_inc(idx, 0, cups.len() - 1);
    cups.insert(idx, picked_up[2]);
    assert_eq!(cups.len(), 9);
}

fn select_destination_cup(cups: &CupsSlice, current_cup_label: Cup) -> (Cup, usize) {
    assert_eq!(cups.len(), 6);
    let mut cup_label = current_cup_label;
    let destination_cup_label = loop {
        cup_label = wrap_dec(cup_label.into(), 1, 9) as Cup;

        if cups.contains(&cup_label) {
            break cup_label;
        }
    };
    let destination_cup_idx = index_from_cup_label(&cups, destination_cup_label);
    (destination_cup_label, destination_cup_idx)
}

fn select_new_current_cup_idx(cups: &CupsSlice, current_cup_label: Cup) -> usize {
    assert_eq!(cups.len(), 9);
    let current_cup_index = index_from_cup_label(&cups, current_cup_label);
    wrap_inc(current_cup_index, 0, 8)
}

fn pickup_cups(cups: &mut Cups, current_cup_index: usize) -> Cups {
    let mut picked_up = Vec::new();
    assert_eq!(cups.len(), 9);
    picked_up.push(cups.remove(wrap_inc(current_cup_index, 0, cups.len() - 1)));
    assert_eq!(cups.len(), 8);
    picked_up.push(cups.remove(wrap_inc(current_cup_index, 0, cups.len() - 2)));
    assert_eq!(cups.len(), 7);
    picked_up.push(cups.remove(wrap_inc(current_cup_index, 0, cups.len() - 3)));
    assert_eq!(cups.len(), 6);
    picked_up
}

fn index_from_cup_label(cups: &CupsSlice, cup_label: Cup) -> usize {
    cups.iter().position(|c| c == &cup_label).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn readme_example_all() {
        let cups = vec![3, 8, 9, 1, 2, 5, 4, 6, 7];
        println!("{}", part_1(cups));

        todo!("{}", 67384529);

        // assert_eq!(part_1(cups), 67384529);
    }

    #[test]
    fn readme_example_manual() {
        let cups = [3, 8, 9, 1, 2, 5, 4, 6, 7].to_vec();

        // -- move 1 --
        // cups: (3) 8  9  1  2  5  4  6  7
        // pick up: 8, 9, 1
        // destination: 2
        let (cups, current_cup_index) = make_move(cups, 0, 1);
        assert_eq!(&cups, &vec![3, 2, 8, 9, 1, 5, 4, 6, 7]);
        assert_eq!(current_cup_index, 1);

        // -- move 2 --
        // cups:  3 (2) 8  9  1  5  4  6  7
        // pick up: 8, 9, 1
        // destination: 7
        let (cups, current_cup_index) = make_move(cups, 1, 2);
        assert_eq!(&cups, &vec![3, 2, 5, 4, 6, 7, 8, 9, 1]);
        assert_eq!(current_cup_index, 2);

        // -- move 3 --
        // cups:  3  2 (5) 4  6  7  8  9  1
        // pick up: 4, 6, 7
        // destination: 3
        let (cups, current_cup_index) = make_move(cups, 8, 3);
        assert_eq!(&cups, &vec![7, 2, 5, 8, 9, 1, 3, 4, 6]);
        assert_eq!(current_cup_index, 3);

        // -- move 4 --
        // cups:  7  2  5 (8) 9  1  3  4  6
        // pick up: 9, 1, 3
        // destination: 7

        // -- move 5 --
        // cups:  3  2  5  8 (4) 6  7  9  1
        // pick up: 6, 7, 9
        // destination: 3
        //
        // -- move 6 --
        // cups:  9  2  5  8  4 (1) 3  6  7
        // pick up: 3, 6, 7
        // destination: 9
        //
        // -- move 7 --
        // cups:  7  2  5  8  4  1 (9) 3  6
        // pick up: 3, 6, 7
        // destination: 8
        //
        // -- move 8 --
        // cups:  8  3  6  7  4  1  9 (2) 5
        // pick up: 5, 8, 3
        // destination: 1
        //
        // -- move 9 --
        // cups:  7  4  1  5  8  3  9  2 (6)
        // pick up: 7, 4, 1
        // destination: 5
        //
        // -- move 10 --
        // cups: (5) 7  4  1  8  3  9  2  6
        // pick up: 7, 4, 1
        // destination: 3
        //
        // -- final --
        // cups:  5 (8) 3  7  4  1  9  2  6

        todo!();
    }

    #[test]
    fn test_wrap() {
        let min = 0;
        let max = 8;

        assert_eq!(wrap_inc(7, min, max), 8);
        assert_eq!(wrap_inc(8, min, max), 0);

        assert_eq!(wrap_dec(1, min, max), 0);
        assert_eq!(wrap_dec(0, min, max), 8);
    }

    #[test]
    fn test_pickup_cups() {
        let mut cups = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        //                  ^
        let picked_up = pickup_cups(&mut cups, 0);
        assert_eq!(&picked_up, &vec![2, 3, 4]);
        assert_eq!(&cups, &vec![1, 5, 6, 7, 8, 9]);

        let mut cups = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        //                                          ^
        let picked_up = pickup_cups(&mut cups, 8);
        assert_eq!(&picked_up, &vec![1, 2, 3]);
        assert_eq!(&cups, &vec![4, 5, 6, 7, 8, 9]);

        let mut cups = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        //                                       ^
        let picked_up = pickup_cups(&mut cups, 7);
        assert_eq!(&picked_up, &vec![9, 1, 2]);
        assert_eq!(&cups, &vec![3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_select_new_current_cup_idx() {
        let cups = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        //              ^  ^
        //              |   ` new (1)
        //               `old (0)

        let old_cup_label = 1;
        let new_cup_label = cups[select_new_current_cup_idx(&cups, old_cup_label)];

        assert_eq!(new_cup_label, 2);
    }

    #[test]
    fn test_place_cups() {
        let mut cups = vec![4, 5, 6, 7, 8, 9];
        place_cups(&mut cups, vec![1, 2, 3], 8);
        assert_eq!(cups, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
