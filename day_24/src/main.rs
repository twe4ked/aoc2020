use std::collections::HashSet;

fn main() {
    let input = include_str!("../input");

    let part_1 = part_1(&input);
    assert_eq!(part_1, 549);
    println!("Part 1: {}", part_1);
}

// https://www.redblobgames.com/grids/hexagons/

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Vec3 {
    x: isize,
    y: isize,
    z: isize,
}

impl Vec3 {
    fn new(x: isize, y: isize, z: isize) -> Self {
        let new = Self { x, y, z };
        new.assert_valid();
        new
    }

    fn assert_valid(&self) {
        assert_eq!(self.x + self.y + self.z, 0);
    }

    fn move_se(&mut self) {
        self.y -= 1;
        self.z += 1;
        self.assert_valid();
    }

    fn move_nw(&mut self) {
        self.y += 1;
        self.z -= 1;
        self.assert_valid();
    }

    fn move_sw(&mut self) {
        self.x -= 1;
        self.z += 1;
        self.assert_valid();
    }

    fn move_ne(&mut self) {
        self.x += 1;
        self.z -= 1;
        self.assert_valid();
    }

    fn move_w(&mut self) {
        self.x -= 1;
        self.y += 1;
        self.assert_valid();
    }

    fn move_e(&mut self) {
        self.x += 1;
        self.y -= 1;
        self.assert_valid();
    }
}

fn part_1(input: &str) -> usize {
    let mut tiles = HashSet::new();

    for line in input.lines() {
        let mut chars = line.chars();
        let mut position = Vec3::new(0, 0, 0);

        loop {
            // E, SE, SW, W, NW, NE
            match chars.next() {
                Some('e') => position.move_e(),
                Some('s') => match chars.next() {
                    Some('e') => position.move_se(),
                    Some('w') => position.move_sw(),
                    _ => panic!("bad char following s"),
                },
                Some('w') => position.move_w(),
                Some('n') => match chars.next() {
                    Some('e') => position.move_ne(),
                    Some('w') => position.move_nw(),
                    _ => panic!("bad char following n"),
                },
                None => break,
                _ => panic!("bad char"),
            }
        }

        if tiles.contains(&position) {
            tiles.remove(&position);
        } else {
            tiles.insert(position.clone());
        }
    }

    tiles.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn readme_example() {
        let input = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";

        assert_eq!(part_1(&input), 10);
    }
}
