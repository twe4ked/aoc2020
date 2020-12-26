use std::collections::HashSet;

fn main() {
    let input = include_str!("../input");

    let part_1 = part_1(&input);
    assert_eq!(part_1, 549);
    println!("Part 1: {}", part_1);

    let part_2 = part_2(&input);
    assert_eq!(part_2, 4147);
    println!("Part 2: {}", part_2);
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

    fn try_new(x: isize, y: isize, z: isize) -> Option<Self> {
        if x + y + z == 0 {
            Some(Self { x, y, z })
        } else {
            None
        }
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

    fn se(&self) -> Self {
        let new = Self {
            x: self.x,
            y: self.y - 1,
            z: self.z + 1,
        };
        new.assert_valid();
        new
    }

    fn nw(&self) -> Self {
        let new = Self {
            x: self.x,
            y: self.y + 1,
            z: self.z - 1,
        };
        new.assert_valid();
        new
    }

    fn sw(&self) -> Self {
        let new = Self {
            x: self.x - 1,
            y: self.y,
            z: self.z + 1,
        };
        new.assert_valid();
        new
    }

    fn ne(&self) -> Self {
        let new = Self {
            x: self.x + 1,
            y: self.y,
            z: self.z - 1,
        };
        new.assert_valid();
        new
    }

    fn w(&self) -> Self {
        let new = Self {
            x: self.x - 1,
            y: self.y + 1,
            z: self.z,
        };
        new.assert_valid();
        new
    }

    fn e(&self) -> Self {
        let new = Self {
            x: self.x + 1,
            y: self.y - 1,
            z: self.z,
        };
        new.assert_valid();
        new
    }
}

fn part_1(input: &str) -> usize {
    flip_tiles(input).len()
}

fn part_2(input: &str) -> usize {
    let mut world = flip_tiles(input);

    for _ in 0..100 {
        let old = world.clone();

        let x_min = world.iter().map(|t| t.x).min().unwrap() - 1;
        let x_max = world.iter().map(|t| t.x).max().unwrap() + 1;
        let y_min = world.iter().map(|t| t.y).min().unwrap() - 1;
        let y_max = world.iter().map(|t| t.y).max().unwrap() + 1;
        let z_min = world.iter().map(|t| t.z).min().unwrap() - 1;
        let z_max = world.iter().map(|t| t.z).max().unwrap() + 1;

        for x in x_min..=x_max {
            for y in y_min..=y_max {
                for z in z_min..=z_max {
                    if let Some(tile) = Vec3::try_new(x, y, z) {
                        let count = neighbors(&old, &tile);

                        // Any black tile with zero or more than 2 black tiles immediately adjacent to it is
                        // flipped to white.
                        if world.contains(&tile) {
                            if count == 0 || count > 2 {
                                world.remove(&tile);
                            }
                        // Any white tile with exactly 2 black tiles immediately adjacent to it is flipped to
                        // black.
                        } else if count == 2 {
                            world.insert(tile);
                        }
                    }
                }
            }
        }
    }

    world.len()
}

fn neighbors(world: &HashSet<Vec3>, tile: &Vec3) -> usize {
    vec![
        world.contains(&tile.se()),
        world.contains(&tile.nw()),
        world.contains(&tile.sw()),
        world.contains(&tile.ne()),
        world.contains(&tile.w()),
        world.contains(&tile.e()),
    ]
    .iter()
    .filter(|b| **b)
    .count()
}

fn flip_tiles(input: &str) -> HashSet<Vec3> {
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

    tiles
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
        assert_eq!(part_2(&input), 2208);
    }
}
