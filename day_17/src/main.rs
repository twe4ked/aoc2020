// --- Day 17: Conway Cubes ---
//
// As your flight slowly drifts through the sky, the Elves at the Mythical Information Bureau at
// the North Pole contact you. They'd like some help debugging a malfunctioning experimental energy
// source aboard one of their super-secret imaging satellites.
//
// The experimental energy source is based on cutting-edge technology: a set of Conway Cubes
// contained in a pocket dimension! When you hear it's having problems, you can't help but agree to
// take a look.
//
// The pocket dimension contains an infinite 3-dimensional grid. At every integer 3-dimensional
// coordinate (x,y,z), there exists a single cube which is either active or inactive.
//
// In the initial state of the pocket dimension, almost all cubes start inactive. The only
// exception to this is a small flat region of cubes (your puzzle input); the cubes in this region
// start in the specified active (#) or inactive (.) state.
//
// The energy source then proceeds to boot up by executing six cycles.
//
// Each cube only ever considers its neighbors: any of the 26 other cubes where any of their
// coordinates differ by at most 1. For example, given the cube at x=1,y=2,z=3, its neighbors
// include the cube at x=2,y=2,z=2, the cube at x=0,y=2,z=3, and so on.
//
// During a cycle, all cubes simultaneously change their state according to the following rules:
//
//     If a cube is active and exactly 2 or 3 of its neighbors are also active, the cube remains
//      active. Otherwise, the cube becomes inactive.
//     If a cube is inactive but exactly 3 of its neighbors are active, the cube becomes active.
//      Otherwise, the cube remains inactive.
//
// The engineers responsible for this experimental energy source would like you to simulate the
// pocket dimension and determine what the configuration of cubes should be at the end of the
// six-cycle boot process.
//
// For example, consider the following initial state:
//
// .#.
// ..#
// ###
//
// Even though the pocket dimension is 3-dimensional, this initial state represents a small
// 2-dimensional slice of it. (In particular, this initial state defines a 3x3x1 region of the
// 3-dimensional space.)
//
// Simulating a few cycles from this initial state produces the following configurations, where the
// result of each cycle is shown layer-by-layer at each given z coordinate (and the frame of view
// follows the active cells in each cycle):
//
// Before any cycles:
//
// z=0
// .#.
// ..#
// ###
//
// After 1 cycle:
//
// z=-1
// #..
// ..#
// .#.
//
// z=0
// #.#
// .##
// .#.
//
// z=1
// #..
// ..#
// .#.
//
//
// After 2 cycles:
//
// z=-2
// .....
// .....
// ..#..
// .....
// .....
//
// z=-1
// ..#..
// .#..#
// ....#
// .#...
// .....
//
// z=0
// ##...
// ##...
// #....
// ....#
// .###.
//
// z=1
// ..#..
// .#..#
// ....#
// .#...
// .....
//
// z=2
// .....
// .....
// ..#..
// .....
// .....
//
// After 3 cycles:
//
// z=-2
// .......
// .......
// ..##...
// ..###..
// .......
// .......
// .......
//
// z=-1
// ..#....
// ...#...
// #......
// .....##
// .#...#.
// ..#.#..
// ...#...
//
// z=0
// ...#...
// .......
// #......
// .......
// .....##
// .##.#..
// ...#...
//
// z=1
// ..#....
// ...#...
// #......
// .....##
// .#...#.
// ..#.#..
// ...#...
//
// z=2
// .......
// .......
// ..##...
// ..###..
// .......
// .......
// .......
//
// After the full six-cycle boot process completes, 112 cubes are left in the active state.
//
// Starting with your given initial configuration, simulate six cycles. How many cubes are left in
// the active state after the sixth cycle?

#[rustfmt::skip]
const INPUT: [&str; 8] = [
    "#.##.##.",
    ".##..#..",
    "....#..#",
    ".##....#",
    "#..##...",
    ".###..#.",
    "..#.#..#",
    ".....#..",
];

#[rustfmt::skip]
const OFFSETS: [(isize, isize); 9] = [
    (-1, -1), (-1, 0), (-1, 1),
    ( 0, -1), ( 0, 0), ( 0, 1),
    ( 1, -1), ( 1, 0), ( 1, 1),
];

use std::collections::HashMap;

fn main() {
    let part_1 = part_1(&INPUT);
    assert_eq!(part_1, 273);
    println!("Part 1: {}", part_1);
}

fn part_1(input: &[&str]) -> usize {
    (0..6).fold(Universe::new(&input), Universe::cycle).active()
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Vec3 {
    x: isize,
    y: isize,
    z: isize,
}

impl Vec3 {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }
}

#[derive(Clone)]
struct Universe {
    cells: HashMap<Vec3, bool>,
}

impl Universe {
    fn new(input: &[&str]) -> Self {
        let mut cells = HashMap::new();
        for (y, line) in input.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                cells.insert(
                    Vec3::new(x as _, y as _, 0),
                    match c {
                        '#' => true,
                        '.' => false,
                        _ => panic!("invalid char"),
                    },
                );
            }
        }
        Self { cells }
    }

    fn bounds(&self) -> (isize, isize, isize, isize, isize, isize) {
        let x_min = self.cells.keys().map(|c| c.x).min().unwrap() - 1;
        let x_max = self.cells.keys().map(|c| c.x).max().unwrap() + 1;
        let y_min = self.cells.keys().map(|c| c.y).min().unwrap() - 1;
        let y_max = self.cells.keys().map(|c| c.y).max().unwrap() + 1;
        let z_min = self.cells.keys().map(|c| c.z).min().unwrap() - 1;
        let z_max = self.cells.keys().map(|c| c.z).max().unwrap() + 1;

        (x_min, x_max, y_min, y_max, z_min, z_max)
    }

    fn cycle(mut self, _: usize) -> Self {
        let old = self.clone();

        let (x_min, x_max, y_min, y_max, z_min, z_max) = self.bounds();

        for z in z_min..=z_max {
            for y in y_min..=y_max {
                for x in x_min..=x_max {
                    let idx = Vec3::new(x, y, z);
                    let count = old.neighbors(&idx);

                    // If a cube is active and exactly 2 or 3 of its neighbors are also active, the
                    // cube remains active. Otherwise, the cube becomes inactive.
                    if *old.cells.get(&idx).unwrap_or(&false) && count != 2 && count != 3 {
                        self.cells.insert(idx, false);
                    // If a cube is inactive but exactly 3 of its neighbors are active, the cube
                    // becomes active. Otherwise, the cube remains inactive.
                    } else if count == 3 {
                        self.cells.insert(idx, true);
                    }
                }
            }
        }

        self
    }

    fn active(&self) -> usize {
        self.cells.values().filter(|&&c| c).count()
    }

    fn neighbors(&self, cell: &Vec3) -> usize {
        let mut count = 0;
        for z in -1..=1 {
            for (x, y) in &OFFSETS {
                let idx = Vec3::new(cell.x + *x, cell.y + *y, cell.z + z);

                if cell != &idx && *self.cells.get(&idx).unwrap_or(&false) {
                    count += 1;
                }
            }
        }
        count
    }
}

impl std::fmt::Debug for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let (x_min, x_max, y_min, y_max, z_min, z_max) = self.bounds();

        for z in (z_min + 1)..z_max {
            writeln!(f, "z={}", z)?;

            for y in (y_min + 1)..y_max {
                for x in (x_min + 1)..x_max {
                    if *self.cells.get(&Vec3::new(x, y, z)).unwrap_or(&false) {
                        write!(f, "#")?;
                    } else {
                        write!(f, ".")?;
                    }
                }
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn readme_example() {
        #[rustfmt::skip]
        let input: [&str; 3] = [
            ".#.",
            "..#",
            "###",
        ];

        assert_eq!(part_1(&input), 112);
    }

    #[test]
    fn test_neighbors() {
        #[rustfmt::skip]
        let input: [&str; 3] = [
            "#..",
            "...",
            "...",
        ];

        let universe = Universe::new(&input);

        assert_eq!(universe.neighbors(&Vec3::new(0, 0, 0)), 0);
        assert_eq!(universe.neighbors(&Vec3::new(1, 0, 0)), 1);
        assert_eq!(universe.neighbors(&Vec3::new(0, 1, 0)), 1);
        assert_eq!(universe.neighbors(&Vec3::new(0, 0, 1)), 1);
    }
}
