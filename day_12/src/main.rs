// --- Day 12: Rain Risk ---
//
// Your ferry made decent progress toward the island, but the storm came in faster than anyone
// expected. The ferry needs to take evasive actions!
//
// Unfortunately, the ship's navigation computer seems to be malfunctioning; rather than giving a
// route directly to safety, it produced extremely circuitous instructions. When the captain uses
// the PA system to ask if anyone can help, you quickly volunteer.
//
// The navigation instructions (your puzzle input) consists of a sequence of single-character
// actions paired with integer input values. After staring at them for a few minutes, you work out
// what they probably mean:
//
//     Action N means to move north by the given value.
//     Action S means to move south by the given value.
//     Action E means to move east by the given value.
//     Action W means to move west by the given value.
//     Action L means to turn left the given number of degrees.
//     Action R means to turn right the given number of degrees.
//     Action F means to move forward by the given value in the direction
//      the ship is currently facing.
//
// The ship starts by facing east. Only the L and R actions change the direction the ship is
// facing. (That is, if the ship is facing east and the next instruction is N10, the ship would
// move north 10 units, but would still move east if the following action were F.)
//
// For example:
//
// F10
// N3
// F7
// R90
// F11
//
// These instructions would be handled as follows:
//
//     F10 would move the ship 10 units east (because the ship starts by facing east) to east 10,
//          north 0.
//     N3 would move the ship 3 units north to east 10, north 3.
//     F7 would move the ship another 7 units east (because the ship is still facing east) to east
//          17, north 3.
//     R90 would cause the ship to turn right by 90 degrees and face south; it remains at east 17,
//          north 3.
//     F11 would move the ship 11 units south to east 17, south 8.
//
// At the end of these instructions, the ship's Manhattan distance (sum of the absolute values of
// its east/west position and its north/south position) from its starting position is 17 + 8 = 25.
//
// Figure out where the navigation instructions lead. What is the Manhattan distance between that
// location and the ship's starting position?

use std::convert::TryInto;

fn main() {
    let input = include_str!("../input");

    let part_1 = part_1(&input);
    assert_eq!(part_1, 1319);
    println!("Part 1: {}", part_1);

    let part_2 = part_2(&input);
    assert_eq!(part_2, 62434);
    println!("Part 2: {}", part_2);
}

fn part_1(input: &str) -> usize {
    parse(input)
        .iter()
        .fold(Ship::new(), handle_action)
        .manhattan_distance()
}

fn part_2(input: &str) -> usize {
    // 10 units east and 1 unit north
    let waypoint = Waypoint::new(10, -1);

    let (ship, _waypoint) = parse(input)
        .iter()
        .fold((Ship::new(), waypoint), handle_action_with_waypoint);

    ship.manhattan_distance()
}

#[derive(Debug)]
struct Waypoint {
    x: isize,
    y: isize,
}

impl Waypoint {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn right(&mut self, degrees: usize) {
        let (x, y) = (0..)
            .take(degrees / 90)
            .fold((self.x, self.y), |(x, y), _| (-y, x));

        self.x = x;
        self.y = y;
    }

    fn left(&mut self, degrees: usize) {
        let (x, y) = (0..)
            .take(degrees / 90)
            .fold((self.x, self.y), |(x, y), _| (y, -x));

        self.x = x;
        self.y = y;
    }
}

#[derive(Debug, Clone)]
struct Ship {
    x: isize,
    y: isize,
    direction: Direction,
}

fn handle_action(mut ship: Ship, action: &Action) -> Ship {
    match action {
        Action::MoveNorth(amount) => ship.y -= amount,
        Action::MoveSouth(amount) => ship.y += amount,
        Action::MoveEast(amount) => ship.x += amount,
        Action::MoveWest(amount) => ship.x -= amount,
        Action::TurnLeft(degrees) => ship.direction.left(*degrees),
        Action::TurnRight(degrees) => ship.direction.right(*degrees),
        Action::MoveForward(amount) => match ship.direction {
            Direction::N => ship.y -= amount,
            Direction::E => ship.x += amount,
            Direction::S => ship.y += amount,
            Direction::W => ship.x -= amount,
        },
    }

    ship
}

fn handle_action_with_waypoint(input: (Ship, Waypoint), action: &Action) -> (Ship, Waypoint) {
    let (mut ship, mut waypoint) = input;

    match action {
        Action::MoveNorth(amount) => waypoint.y -= amount,
        Action::MoveSouth(amount) => waypoint.y += amount,
        Action::MoveEast(amount) => waypoint.x += amount,
        Action::MoveWest(amount) => waypoint.x -= amount,
        Action::TurnLeft(degrees) => waypoint.left(*degrees),
        Action::TurnRight(degrees) => waypoint.right(*degrees),
        Action::MoveForward(amount) => {
            ship.x += amount * waypoint.x;
            ship.y += amount * waypoint.y;
        }
    }

    (ship, waypoint)
}

impl Ship {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            direction: Direction::E,
        }
    }

    fn manhattan_distance(&self) -> usize {
        (self.x.abs() + self.y.abs()).try_into().unwrap()
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    N,
    S,
    E,
    W,
}

impl Direction {
    fn prev(&self) -> Self {
        match self {
            Direction::N => Direction::W,
            Direction::W => Direction::S,
            Direction::S => Direction::E,
            Direction::E => Direction::N,
        }
    }

    fn next(&self) -> Self {
        match self {
            Direction::N => Direction::E,
            Direction::E => Direction::S,
            Direction::S => Direction::W,
            Direction::W => Direction::N,
        }
    }

    fn iter(&self) -> DirectionIter {
        DirectionIter(*self)
    }

    fn right(&mut self, degrees: usize) {
        *self = self.iter().take(degrees / 90).last().unwrap();
    }

    fn left(&mut self, degrees: usize) {
        *self = self.iter().rev().take(degrees / 90).last().unwrap();
    }
}

struct DirectionIter(Direction);

impl Iterator for DirectionIter {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        self.0 = self.0.next();
        Some(self.0)
    }
}

impl DoubleEndedIterator for DirectionIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0 = self.0.prev();
        Some(self.0)
    }
}

#[derive(Debug)]
enum Action {
    MoveNorth(isize),
    MoveSouth(isize),
    MoveEast(isize),
    MoveWest(isize),
    TurnLeft(usize),
    TurnRight(usize),
    MoveForward(isize),
}

fn parse(input: &str) -> Vec<Action> {
    input
        .lines()
        .map(|line| {
            let (action, value) = line.split_at(1);
            let value: isize = value.parse().unwrap();

            match action {
                "N" => Action::MoveNorth(value),
                "S" => Action::MoveSouth(value),
                "E" => Action::MoveEast(value),
                "W" => Action::MoveWest(value),
                "L" => Action::TurnLeft(value.try_into().unwrap()),
                "R" => Action::TurnRight(value.try_into().unwrap()),
                "F" => Action::MoveForward(value),
                _ => panic!("invalid action"),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn readme_example() {
        let input = "F10
N3
F7
R90
F11";

        assert_eq!(part_1(&input), 25);
        assert_eq!(part_2(&input), 286);
    }
}
