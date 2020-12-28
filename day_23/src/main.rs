fn main() {
    let part_1 = part_1(vec![7, 8, 4, 2, 3, 5, 9, 1, 6]);
    // assert_eq!(part_1, 99);
    println!("Part 1: {}", part_1);
}

fn part_1(input: Cups) -> u64 {
    let mut cups = input;
    let mut current_cup_index = 0;
    for move_number in 1..=100 {
        let (new_cups, new_index) = make_move(cups, current_cup_index, move_number);
        cups = new_cups;
        current_cup_index = new_index;
    }

    println!("{:?}", cups);

    1
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

type Cups = Vec<u8>;
type CupsSlice = [u8];

fn print_cups(cups: &CupsSlice, current_cup_label: u8) {
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
    dbg!(current_cup_label, current_cup_index);

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
    let (destination_cup_label, destination_cup_index) =
        select_destination_cup(&cups, current_cup_label);
    println!(
        "destination: {} (idx: {})",
        destination_cup_label, destination_cup_index
    );
    println!();

    // The crab places the cups it just picked up so that they are immediately clockwise of the
    // destination cup. They keep the same order as when they were picked up.
    assert_eq!(cups.len(), 6);
    cups.insert(wrap_add(destination_cup_index, 1, 0, 5), picked_up[0]);
    assert_eq!(cups.len(), 7);
    cups.insert(wrap_add(destination_cup_index, 2, 0, 6), picked_up[1]);
    assert_eq!(cups.len(), 8);
    cups.insert(wrap_add(destination_cup_index, 3, 0, 7), picked_up[2]);

    let length = cups.len();
    assert_eq!(length, 9);

    // The crab selects a new current cup: the cup which is immediately clockwise of the current
    // cup.
    let new_current_cup_idx = select_new_current_cup_idx(&cups, current_cup_label);

    (cups, new_current_cup_idx)
}

fn select_destination_cup(cups: &CupsSlice, current_cup_label: u8) -> (u8, usize) {
    assert_eq!(cups.len(), 6);
    let mut cup_label = current_cup_label;
    let destination_cup_label = loop {
        cup_label = wrap_dec(cup_label.into(), 1, 9) as u8;

        if cups.contains(&cup_label) {
            break cup_label;
        }
    };
    let destination_cup_index = index_from_cup_label(&cups, destination_cup_label);
    (destination_cup_label, destination_cup_index)
}

fn select_new_current_cup_idx(cups: &CupsSlice, current_cup_label: u8) -> usize {
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

fn index_from_cup_label(cups: &CupsSlice, cup_label: u8) -> usize {
    cups.iter().position(|c| c == &cup_label).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn readme_example() {
        let cups = vec![3, 8, 9, 1, 2, 5, 4, 6, 7];
        part_1(cups);

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
        let (cups, current_cup_index) = make_move(cups, 0, 2);
        assert_eq!(&cups, &vec![3, 2, 5, 4, 6, 7, 8, 9, 1]);
        assert_eq!(current_cup_index, 2);

        // -- move 3 --
        // cups:  3  2 (5) 4  6  7  8  9  1
        // pick up: 4, 6, 7
        // destination: 3
        let (cups, current_cup_index) = make_move(cups, 0, 3);
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
}
