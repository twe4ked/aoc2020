use std::collections::HashMap;

fn main() {
    let lines: Vec<_> = include_str!("../input").lines().collect();

    let part_1 = part_1(&lines);
    assert_eq!(part_1, 9628746976360);
    println!("Part 1: {}", part_1);
}

#[derive(Default)]
struct State {
    zer_mask: usize,
    one_mask: usize,
    mem: HashMap<usize, usize>,
}

impl State {
    fn apply_bitmasks(&self, value: usize) -> usize {
        (value & self.zer_mask) | self.one_mask
    }
}

fn part_1(lines: &[&str]) -> usize {
    lines
        .iter()
        .fold(State::default(), |mut state, line| {
            match line.chars().nth(1).unwrap() {
                // mask
                //  ^
                'a' => {
                    let (zer_mask, one_mask) = create_bitmasks(line.split(" = ").nth(1).unwrap());
                    state.zer_mask = zer_mask;
                    state.one_mask = one_mask;
                }
                // mem
                //  ^
                'e' => {
                    let (index, value) = parse_mem_line(line);
                    state.mem.insert(index, state.apply_bitmasks(value));
                }
                _ => panic!("bad line"),
            }
            state
        })
        .mem
        .values()
        .sum()
}

fn parse_mem_line(line: &&str) -> (usize, usize) {
    let parts = line.split('[').last().unwrap();
    let mut parts = parts.split(']');

    let index = parts.next().unwrap().parse().unwrap();

    let value = parts
        .next()
        .unwrap()
        .split(" = ")
        .last()
        .unwrap()
        .parse()
        .unwrap();

    (index, value)
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