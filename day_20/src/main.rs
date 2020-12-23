// --- Day 20: Jurassic Jigsaw ---
//
// The high-speed train leaves the forest and quickly carries you south. You can even see a desert
// in the distance! Since you have some spare time, you might as well see if there was anything
// interesting in the image the Mythical Information Bureau satellite captured.
//
// After decoding the satellite messages, you discover that the data actually contains many small
// images created by the satellite's camera array. The camera array consists of many cameras;
// rather than produce a single square image, they produce many smaller square image tiles that
// need to be reassembled back into a single image.
//
// Each camera in the camera array returns a single monochrome image tile with a random unique ID
// number. The tiles (your puzzle input) arrived in a random order.
//
// Worse yet, the camera array appears to be malfunctioning: each image tile has been rotated and
// flipped to a random orientation. Your first task is to reassemble the original image by
// orienting the tiles so they fit together.
//
// To show how the tiles should be reassembled, each tile's image data includes a border that
// should line up exactly with its adjacent tiles. All tiles have this border, and the border lines
// up exactly when the tiles are both oriented correctly. Tiles at the edge of the image also have
// this border, but the outermost edges won't line up with any other tiles.
//
// For example, suppose you have the following nine tiles:
//
// Tile 2311:
// ..##.#..#.
// ##..#.....
// #...##..#.
// ####.#...#
// ##.##.###.
// ##...#.###
// .#.#.#..##
// ..#....#..
// ###...#.#.
// ..###..###
//
// Tile 1951:
// #.##...##.
// #.####...#
// .....#..##
// #...######
// .##.#....#
// .###.#####
// ###.##.##.
// .###....#.
// ..#.#..#.#
// #...##.#..
//
// Tile 1171:
// ####...##.
// #..##.#..#
// ##.#..#.#.
// .###.####.
// ..###.####
// .##....##.
// .#...####.
// #.##.####.
// ####..#...
// .....##...
//
// Tile 1427:
// ###.##.#..
// .#..#.##..
// .#.##.#..#
// #.#.#.##.#
// ....#...##
// ...##..##.
// ...#.#####
// .#.####.#.
// ..#..###.#
// ..##.#..#.
//
// Tile 1489:
// ##.#.#....
// ..##...#..
// .##..##...
// ..#...#...
// #####...#.
// #..#.#.#.#
// ...#.#.#..
// ##.#...##.
// ..##.##.##
// ###.##.#..
//
// Tile 2473:
// #....####.
// #..#.##...
// #.##..#...
// ######.#.#
// .#...#.#.#
// .#########
// .###.#..#.
// ########.#
// ##...##.#.
// ..###.#.#.
//
// Tile 2971:
// ..#.#....#
// #...###...
// #.#.###...
// ##.##..#..
// .#####..##
// .#..####.#
// #..#.#..#.
// ..####.###
// ..#.#.###.
// ...#.#.#.#
//
// Tile 2729:
// ...#.#.#.#
// ####.#....
// ..#.#.....
// ....#..#.#
// .##..##.#.
// .#.####...
// ####.#.#..
// ##.####...
// ##..#.##..
// #.##...##.
//
// Tile 3079:
// #.#.#####.
// .#..######
// ..#.......
// ######....
// ####.#..#.
// .#...#.##.
// #.#####.##
// ..#.###...
// ..#.......
// ..#.###...
//
// By rotating, flipping, and rearranging them, you can find a square arrangement that causes all
// adjacent borders to line up:
//
// #...##.#.. ..###..### #.#.#####.
// ..#.#..#.# ###...#.#. .#..######
// .###....#. ..#....#.. ..#.......
// ###.##.##. .#.#.#..## ######....
// .###.##### ##...#.### ####.#..#.
// .##.#....# ##.##.###. .#...#.##.
// #...###### ####.#...# #.#####.##
// .....#..## #...##..#. ..#.###...
// #.####...# ##..#..... ..#.......
// #.##...##. ..##.#..#. ..#.###...
//
// #.##...##. ..##.#..#. ..#.###...
// ##..#.##.. ..#..###.# ##.##....#
// ##.####... .#.####.#. ..#.###..#
// ####.#.#.. ...#.##### ###.#..###
// .#.####... ...##..##. .######.##
// .##..##.#. ....#...## #.#.#.#...
// ....#..#.# #.#.#.##.# #.###.###.
// ..#.#..... .#.##.#..# #.###.##..
// ####.#.... .#..#.##.. .######...
// ...#.#.#.# ###.##.#.. .##...####
//
// ...#.#.#.# ###.##.#.. .##...####
// ..#.#.###. ..##.##.## #..#.##..#
// ..####.### ##.#...##. .#.#..#.##
// #..#.#..#. ...#.#.#.. .####.###.
// .#..####.# #..#.#.#.# ####.###..
// .#####..## #####...#. .##....##.
// ##.##..#.. ..#...#... .####...#.
// #.#.###... .##..##... .####.##.#
// #...###... ..##...#.. ...#..####
// ..#.#....# ##.#.#.... ...##.....
//
// For reference, the IDs of the above tiles are:
//
// 1951    2311    3079
// 2729    1427    2473
// 2971    1489    1171
//
// To check that you've assembled the image correctly, multiply the IDs of the four corner tiles
// together. If you do this with the assembled tiles from the example above, you get 1951 * 3079 *
// 2971 * 1171 = 20899048083289.
//
// Assemble the tiles into an image. What do you get if you multiply together the IDs of the four
// corner tiles?

use bitvec::prelude::*;

fn main() {
    todo!();
}

#[derive(Debug)]
struct Tile {
    id: u64,
    data: BitVec<Msb0, u16>,
}

impl Tile {
    // Top
    fn edge_1(&self) -> u16 {
        self.data.as_raw_slice()[0]
    }

    // Right
    fn edge_2(&self) -> u16 {
        let mut ret = 0;
        for i in 1..11 {
            ret <<= 1;
            if self.data.as_bitslice()[(16 * i) - 1] {
                ret |= 1
            }
        }
        ret
    }

    // Bottom
    fn edge_3(&self) -> u16 {
        self.data.as_raw_slice()[9]
    }

    // Left
    fn edge_4(&self) -> u16 {
        let mut ret = 0;
        for i in 1..11 {
            ret <<= 1;
            if self.data.as_bitslice()[(16 * i) - 10] {
                ret |= 1
            }
        }
        ret
    }
}

fn parse_tile(input: &str) -> Tile {
    let mut parts = input.splitn(2, "\n");
    let id = parts
        .next()
        .unwrap()
        // Tile 3079:
        .strip_suffix(':')
        .unwrap()
        // Tile
        // 3079
        .split(' ')
        .last()
        .unwrap()
        .parse()
        .unwrap();

    let mut data = BitVec::new();
    for line in parts.next().unwrap().lines() {
        // Add 6 zeroed bits to make up the u16
        for _ in 0..6 {
            data.push(false);
        }

        // Add the other 10 bits to represent the row of the tile
        for c in line.chars() {
            match c {
                '#' => data.push(true),
                '.' => data.push(false),
                _ => panic!("bad char: {}", c),
            };
        }
    }

    Tile { id, data }
}

fn parse(input: &str) -> Vec<Tile> {
    input.split("\n\n").map(parse_tile).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";

    #[test]
    fn readme_example() {
        todo!();
    }

    #[test]
    fn test_tile_edges() {
        let tiles = parse(&INPUT);

        // ..##.#..#.
        // ##..#.....
        // #...##..#.
        // ####.#...#
        // ##.##.###.
        // ##...#.###
        // .#.#.#..##
        // ..#....#..
        // ###...#.#.
        // ..###..###
        let first_tile = &tiles[0];

        assert_eq!(first_tile.edge_1(), 0b0011010010);
        assert_eq!(first_tile.edge_2(), 0b0001011001);
        assert_eq!(first_tile.edge_3(), 0b0011100111);
        assert_eq!(first_tile.edge_4(), 0b0111110010);
    }
}
