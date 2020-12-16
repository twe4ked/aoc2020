// --- Day 14: Docking Data ---
//
// As your ferry approaches the sea port, the captain asks for your help again. The computer system
// that runs this port isn't compatible with the docking program on the ferry, so the docking
// parameters aren't being correctly initialized in the docking program's memory.
//
// After a brief inspection, you discover that the sea port's computer system uses a strange
// bitmask system in its initialization program. Although you don't have the correct decoder chip
// handy, you can emulate it in software!
//
// The initialization program (your puzzle input) can either update the bitmask or write a value to
// memory. Values and memory addresses are both 36-bit unsigned integers. For example, ignoring
// bitmasks for a moment, a line like mem[8] = 11 would write the value 11 to memory address 8.
//
// The bitmask is always given as a string of 36 bits, written with the most significant bit
// (representing 2^35) on the left and the least significant bit (2^0, that is, the 1s bit) on the
// right. The current bitmask is applied to values immediately before they are written to memory: a
// 0 or 1 overwrites the corresponding bit in the value, while an X leaves the bit in the value
// unchanged.
//
// For example, consider the following program:
//
// mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
// mem[8] = 11
// mem[7] = 101
// mem[8] = 0
//
// This program starts by specifying a bitmask (mask = ....). The mask it specifies will overwrite
// two bits in every written value: the 2s bit is overwritten with 0, and the 64s bit is
// overwritten with 1.
//
// The program then attempts to write the value 11 to memory address 8. By expanding everything out
// to individual bits, the mask is applied as follows:
//
// value:  000000000000000000000000000000001011  (decimal 11)
// mask:   XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
// result: 000000000000000000000000000001001001  (decimal 73)
//
// So, because of the mask, the value 73 is written to memory address 8 instead. Then, the program
// tries to write 101 to address 7:
//
// value:  000000000000000000000000000001100101  (decimal 101)
// mask:   XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
// result: 000000000000000000000000000001100101  (decimal 101)
//
// This time, the mask has no effect, as the bits it overwrote were already the values the mask
// tried to set. Finally, the program tries to write 0 to address 8:
//
// value:  000000000000000000000000000000000000  (decimal 0)
// mask:   XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
// result: 000000000000000000000000000001000000  (decimal 64)
//
// 64 is written to address 8 instead, overwriting the value that was there previously.
//
// To initialize your ferry's docking program, you need the sum of all values left in memory after
// the initialization program completes. (The entire 36-bit address space begins initialized to the
// value 0 at every address.) In the above example, only two values in memory are not zero - 101
// (at address 7) and 64 (at address 8) - producing a sum of 165.
//
// Execute the initialization program. What is the sum of all values left in memory after it
// completes?
//
// Your puzzle answer was 9628746976360.
//
// The first half of this puzzle is complete! It provides one gold star: *
//
// --- Part Two ---
//
// For some reason, the sea port's computer system still can't communicate with your ferry's
// docking program. It must be using version 2 of the decoder chip!
//
// A version 2 decoder chip doesn't modify the values being written at all. Instead, it acts as a
// memory address decoder. Immediately before a value is written to memory, each bit in the bitmask
// modifies the corresponding bit of the destination memory address in the following way:
//
//     If the bitmask bit is 0, the corresponding memory address bit is unchanged.
//     If the bitmask bit is 1, the corresponding memory address bit is overwritten with 1.
//     If the bitmask bit is X, the corresponding memory address bit is floating.
//
// A floating bit is not connected to anything and instead fluctuates unpredictably. In practice,
// this means the floating bits will take on all possible values, potentially causing many memory
// addresses to be written all at once!
//
// For example, consider the following program:
//
// mask = 000000000000000000000000000000X1001X
// mem[42] = 100
// mask = 00000000000000000000000000000000X0XX
// mem[26] = 1
//
// When this program goes to write to memory address 42, it first applies the bitmask:
//
// address: 000000000000000000000000000000101010  (decimal 42)
// mask:    000000000000000000000000000000X1001X
// result:  000000000000000000000000000000X1101X
//
// After applying the mask, four bits are overwritten, three of which are different, and two of
// which are floating. Floating bits take on every possible combination of values; with two
// floating bits, four actual memory addresses are written:
//
// 000000000000000000000000000000011010  (decimal 26)
// 000000000000000000000000000000011011  (decimal 27)
// 000000000000000000000000000000111010  (decimal 58)
// 000000000000000000000000000000111011  (decimal 59)
//
// Next, the program is about to write to memory address 26 with a different bitmask:
//
// address: 000000000000000000000000000000011010  (decimal 26)
// mask:    00000000000000000000000000000000X0XX
// result:  00000000000000000000000000000001X0XX
//
// This results in an address with three floating bits, causing writes to eight memory addresses:
//
// 000000000000000000000000000000010000  (decimal 16)
// 000000000000000000000000000000010001  (decimal 17)
// 000000000000000000000000000000010010  (decimal 18)
// 000000000000000000000000000000010011  (decimal 19)
// 000000000000000000000000000000011000  (decimal 24)
// 000000000000000000000000000000011001  (decimal 25)
// 000000000000000000000000000000011010  (decimal 26)
// 000000000000000000000000000000011011  (decimal 27)
//
// The entire 36-bit address space still begins initialized to the value 0 at every address, and
// you still need the sum of all values left in memory at the end of the program. In this example,
// the sum is 208.
//
// Execute the initialization program using an emulator for a version 2 decoder chip. What is the
// sum of all values left in memory after it completes?

use std::collections::HashMap;

fn main() {
    let lines: Vec<_> = include_str!("../input").lines().collect();

    let part_1 = part_1(&lines);
    assert_eq!(part_1, 9628746976360);
    println!("Part 1: {}", part_1);

    let part_2 = part_2(&lines);
    assert_eq!(part_2, 4574598714592);
    println!("Part 2: {}", part_2);
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
    parse_lines(lines)
        .iter()
        .fold(State::default(), |mut state, line| {
            match line {
                Line::Mask(mask) => {
                    let (zer_mask, one_mask) = create_bitmasks(mask);
                    state.zer_mask = zer_mask;
                    state.one_mask = one_mask;
                }
                Line::Mem(address, value) => {
                    state.mem.insert(*address, state.apply_bitmasks(*value));
                }
            }
            state
        })
        .mem
        .values()
        .sum()
}

#[derive(Default)]
struct StatePart2 {
    mask: String,
    mem: HashMap<usize, usize>,
}

fn part_2(lines: &[&str]) -> usize {
    parse_lines(lines)
        .iter()
        .fold(StatePart2::default(), |mut state, line| {
            match line {
                Line::Mask(mask) => state.mask = mask.to_owned(),
                Line::Mem(address, value) => {
                    let mask = decode_memory_address(&state.mask, *address);
                    for address in mask_to_addresses(mask) {
                        state.mem.insert(address, *value);
                    }
                }
            }
            state
        })
        .mem
        .values()
        .sum()
}

enum Line {
    Mask(String),
    Mem(usize, usize),
}

fn parse_lines(lines: &[&str]) -> Vec<Line> {
    lines
        .iter()
        .map(|line| {
            match line.chars().nth(1).unwrap() {
                // mask
                //  ^
                'a' => Line::Mask(line.split(" = ").nth(1).unwrap().to_string()),
                // mem
                //  ^
                'e' => {
                    let (index, value) = parse_mem_line(line);
                    Line::Mem(index, value)
                }
                _ => panic!("bad line"),
            }
        })
        .collect()
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
        zer_mask <<= 1;
        one_mask <<= 1;

        match c {
            'X' => zer_mask |= 1,
            '1' => {
                zer_mask |= 1;
                one_mask |= 1;
            }
            '0' => {}
            _ => panic!("bad char"),
        }
    }

    (zer_mask, one_mask)
}

// If the bitmask bit is 0, the corresponding memory address bit is unchanged.
// If the bitmask bit is 1, the corresponding memory address bit is overwritten with 1.
// If the bitmask bit is X, the corresponding memory address bit is floating.
fn decode_memory_address(mask: &str, address: usize) -> String {
    let mut result = String::new();

    for (mask_c, addr_c) in mask.chars().zip(format!("{:036b}", address).chars()) {
        match mask_c {
            '0' => result.push(addr_c),
            '1' => result.push('1'),
            'X' => result.push('X'),
            _ => panic!("bad mask"),
        }
    }

    result
}

// 000000000000000000000000000000X1101X
//                               |    |
// 000000000000000000000000000000011010  (decimal 26)
// 000000000000000000000000000000011011  (decimal 27)
// 000000000000000000000000000000111010  (decimal 58)
// 000000000000000000000000000000111011  (decimal 59)
fn mask_to_addresses(mask: String) -> Vec<usize> {
    let mut addresses = Vec::new();

    let mut masks = Vec::new();
    masks.push(mask);

    while !masks.is_empty() {
        let mask = masks.pop().unwrap();

        if let Some(index) = mask.find('X') {
            // If we find an X, add two more masks to the queue with
            // the found X replaced with 1 and 0
            let mut mask_zer = mask.clone();
            mask_zer.replace_range(index..index + 1, "0");
            masks.push(mask_zer);

            let mut mask_one = mask;
            mask_one.replace_range(index..index + 1, "1");
            masks.push(mask_one);
        } else {
            // Otherwise, the address is finished and can be converted to an integer
            addresses.push(usize::from_str_radix(&mask, 2).unwrap())
        }
    }

    addresses
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn readme_example_part_1() {
        let input = vec![
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X",
            "mem[8] = 11",
            "mem[7] = 101",
            "mem[8] = 0",
        ];

        assert_eq!(part_1(&input), 165);
    }

    #[test]
    fn readme_example_part_2() {
        let input = vec![
            "mask = 000000000000000000000000000000X1001X",
            "mem[42] = 100",
            "mask = 00000000000000000000000000000000X0XX",
            "mem[26] = 1",
        ];

        assert_eq!(part_2(&input), 208);
    }

    #[test]
    fn test_decode_memory_address() {
        let address = 0b000000000000000000000000000000101010;
        let mask = "000000000000000000000000000000X1001X".to_string();
        let result = "000000000000000000000000000000X1101X".to_string();

        assert_eq!(decode_memory_address(&mask, address), result);
    }

    #[test]
    fn test_mask_to_addresses() {
        let mask = "000000000000000000000000000000X1101X".to_string();

        assert_eq!(
            mask_to_addresses(mask),
            vec![
                0b000000000000000000000000000000111011, // (decimal 59)
                0b000000000000000000000000000000111010, // (decimal 58)
                0b000000000000000000000000000000011011, // (decimal 27)
                0b000000000000000000000000000000011010, // (decimal 26)
            ]
        );
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
