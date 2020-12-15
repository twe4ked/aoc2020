use std::collections::HashMap;

fn main() {
    let lines: Vec<_> = include_str!("../input").lines().collect();

    let part_1 = part_1(&lines);
    assert_eq!(part_1, 9628746976360);
    println!("Part 1: {}", part_1);
}

fn part_1(lines: &[&str]) -> usize {
    let apply_bitmasks = |value, zer_mask, one_mask| (value & zer_mask) | one_mask;

    let mut mem = HashMap::new();

    let mut zer_mask = 0;
    let mut one_mask = 0;

    // mask = 0111X10100100X1111X10010X000X1000001
    // mem[50907] = 468673978
    // mem[22295] = 3337449
    // mem[58474] = 56418393
    // mem[15362] = 243184
    // mem[65089] = 110688658
    for line in lines {
        match line.chars().nth(1).unwrap() {
            // mask
            //  ^
            'a' => {
                let bitmasks = create_bitmasks(line.split(" = ").nth(1).unwrap());
                zer_mask = bitmasks.0;
                one_mask = bitmasks.1;
            }
            // mem
            //  ^
            'e' => {
                let parts = line.split('[').last().unwrap();
                let mut parts = parts.split(']');

                let index: usize = parts.next().unwrap().parse().unwrap();

                let value: usize = parts
                    .next()
                    .unwrap()
                    .split(" = ")
                    .last()
                    .unwrap()
                    .parse()
                    .unwrap();

                let masked_value = apply_bitmasks(value, zer_mask, one_mask);

                mem.insert(index, masked_value);
            }
            _ => panic!("bad line"),
        }
    }

    mem.values().sum()
}

fn create_bitmasks(mask: &str) -> (usize, usize) {
    let mut zer_mask = 0;
    let mut one_mask = 0;

    for c in mask.chars() {
        match c {
            'X' => {
                zer_mask = zer_mask << 1 | 1;
                one_mask = one_mask << 1 | 0;
            }
            '0' => {
                zer_mask = zer_mask << 1 | 0;
                one_mask = one_mask << 1 | 0;
            }
            '1' => {
                zer_mask = zer_mask << 1 | 1;
                one_mask = one_mask << 1 | 1;
            }
            _ => panic!("bad char"),
        }
    }

    (zer_mask, one_mask)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn readme_example() {
        let input = vec![
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X",
            "mem[8] = 11",
            "mem[7] = 101",
            "mem[8] = 0",
        ];

        assert_eq!(part_1(&input), 165);
    }

    #[test]
    fn test_create_bitmasks() {
        // value:  000000000000000000000000000000001011  (decimal 11)
        // mask:   XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
        // result: 000000000000000000000000000001001001  (decimal 73)

        let mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";

        let (zer_mask, one_mask) = create_bitmasks(mask);

        assert_eq!(zer_mask, 0b111111111111111111111111111111111101);
        assert_eq!(one_mask, 0b000000000000000000000000000001000000);
    }
}
