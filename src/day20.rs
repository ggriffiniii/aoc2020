use crate::split_once;
use aoc_runner_derive::aoc;
use std::collections::HashMap;

#[derive(Debug)]
struct Tile {
    id: usize,
    data: [bool; 100],
}
impl Tile {
    fn parse(input: &str) -> Option<Tile> {
        let (id_line, input_data) = split_once(input, ":\n")?;
        let (_, id) = split_once(id_line, " ")?;
        let mut data = [false; 100];
        for idx in input_data
            .as_bytes()
            .iter()
            .cloned()
            .filter(|&b| b != b'\n')
            .enumerate()
            .filter_map(|(idx, b)| if b == b'#' { Some(idx) } else { None })
        {
            data[idx] = true;
        }
        Some(Tile {
            id: id.parse().ok()?,
            data,
        })
    }

    fn sides(&self) -> Vec<u16> {
        let top = self.data[0..10]
            .iter()
            .cloned()
            .enumerate()
            .filter(|&(_, b)| b)
            .fold(0u16, |accum, (idx, _)| accum | (1 << idx));
        let bottom = self.data[90..100]
            .iter()
            .cloned()
            .enumerate()
            .filter(|&(_, b)| b)
            .fold(0u16, |accum, (idx, _)| accum | (1 << idx));

        let left = self
            .data
            .iter()
            .step_by(10)
            .cloned()
            .enumerate()
            .filter(|&(_, b)| b)
            .fold(0u16, |accum, (idx, _)| accum | (1 << idx));

        let right = self
            .data
            .iter()
            .skip(9)
            .step_by(10)
            .cloned()
            .enumerate()
            .filter(|&(_, b)| b)
            .fold(0u16, |accum, (idx, _)| accum | (1 << idx));

        // Put the id's in a consistent order.
        fn consistent_id(id: u16) -> u16 {
            let reversed = id.reverse_bits() >> 6;
            if id < reversed {
                reversed
            } else {
                id
            }
        }
        vec![
            consistent_id(top),
            consistent_id(right),
            consistent_id(bottom),
            consistent_id(left),
        ]
    }
}

const EXAMPLE: &str = "Tile 2311:
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
..#.###...
";

#[aoc(day20, part1)]
fn solve_d20_p1(input: &str) -> usize {
    let tiles: Vec<_> = input
        .split("\n\n")
        .map(|i| Tile::parse(i).unwrap())
        .collect();
    let mut side_to_tile: HashMap<_, Vec<_>> = HashMap::new();
    for tile in &tiles {
        for side in tile.sides() {
            side_to_tile.entry(side).or_default().push(tile.id);
        }
    }
    tiles
        .iter()
        .filter_map(|tile| {
            let num_unique_sides = tile.sides().iter().fold(0, |accum, side| {
                if side_to_tile.get(&side).unwrap().len() == 1 {
                    accum + 1
                } else {
                    accum
                }
            });
            if num_unique_sides == 2 {
                Some(tile.id)
            } else {
                None
            }
        })
        .product()
}
