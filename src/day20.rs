use crate::split_once;
use aoc_runner_derive::aoc;
use std::collections::HashMap;
use std::collections::VecDeque;

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

    fn top(&self) -> u16 {
        self.data[0..10]
            .iter()
            .cloned()
            .enumerate()
            .filter(|&(_, b)| b)
            .fold(0u16, |accum, (idx, _)| accum | (1 << idx))
    }

    fn bottom(&self) -> u16 {
        self.data[90..100]
            .iter()
            .cloned()
            .enumerate()
            .filter(|&(_, b)| b)
            .fold(0u16, |accum, (idx, _)| accum | (1 << idx))
    }

    fn left(&self) -> u16 {
        self
            .data
            .iter()
            .step_by(10)
            .cloned()
            .enumerate()
            .filter(|&(_, b)| b)
            .fold(0u16, |accum, (idx, _)| accum | (1 << idx))
    }

    fn right(&self) -> u16 {
        self
            .data
            .iter()
            .skip(9)
            .step_by(10)
            .cloned()
            .enumerate()
            .filter(|&(_, b)| b)
            .fold(0u16, |accum, (idx, _)| accum | (1 << idx))
    }

    fn flip(&mut self) {
        let mut update = [false; 100];
        for rowi in 0..10 {
            let flipped_row = 9 - rowi;
            for coli in 0..10 {
                update[flipped_row*10+coli] = self.data[rowi*10+coli];
            }
        }
        self.data = update;
    }

    fn rotate_90(&mut self) {
        /*
            (y,x)
            (0,0) -> (0,9)
            (0,1) -> (1,9)
            (0,2) -> (2,9)
            (0,9) -> (9,9)
            (1,0) -> (0,8)
            (1,1) -> (1,8)
            (1,2) -> (2,8)
            (1,9) -> (9,8)

            (9,7) -> (7,0)
            (9,8) -> (8,0)
            (9,9) -> (9,0)
        */

        let mut update = [false; 100];
        for rowi in 0..10 {
            let new_col = 9 - rowi;
            for coli in 0..10 {
                let new_row = coli;
                update[new_row*10+new_col] = self.data[rowi*10+coli];
            }
        }
    }

    // Sides can be flipped. This puts the bits in a consistent order for
    // comparison sake.
    fn side_id(side: u16) -> u16 {
        let reversed = side.reverse_bits() >> 6;
        if side < reversed {
            reversed
        } else {
            side
        }
    }

    fn sides(&self) -> Vec<u16> {
        vec![self.top(), self.right(), self.bottom(), self.left()]
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
            side_to_tile.entry(Tile::side_id(side)).or_default().push(tile.id);
        }
    }
    tiles
        .iter()
        .filter_map(|tile| {
            let num_unique_sides = tile.sides().iter().fold(0, |accum, side| {
                if side_to_tile.get(&Tile::side_id(*side)).unwrap().len() == 1 {
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

// Rotate and flip the tile in all permutations, stopping if matched ever returns true. Returned
// true if a match was found, false if all permutations were exhausted without a match.
fn rotate_and_flip_until<F>(tile: &mut Tile, matched: F) -> bool
where
    F: Fn(&Tile) -> bool,
{
    for _ in 0 .. 4 {
        if matched(tile) {
            return true;
        }
        tile.rotate_90();
    }
    tile.flip();
    for _ in 0 .. 4 {
        if matched(tile) {
            return true;
        }
        tile.rotate_90();
    }
    false
}

#[aoc(day20, part2)]
fn solve_d20_p2(input: &str) -> usize {
    let input = EXAMPLE;
    let mut tiles: VecDeque<_> = input
        .split("\n\n")
        .map(|i| Tile::parse(i).unwrap())
        .collect();
    let mut side_occurrences: HashMap<_, usize> = HashMap::new();
    for tile in &tiles {
        for side in tile.sides() {
            *side_occurrences.entry(Tile::side_id(side)).or_default() += 1;
        }
    }
    let num_sides = |side| {
        *side_occurrences.get(&Tile::side_id(side)).unwrap()
    };

    let mut grid = Vec::new();

    grid.push(loop {
        let mut tile = tiles.pop_back().unwrap();
        let num_unique_sides = tile.sides().into_iter().filter(|&side| num_sides(side) == 1).count();
        if num_unique_sides != 2 {
            tiles.push_front(tile);
            continue;
        }

        if rotate_and_flip_until(&mut tile, |t| num_sides(t.left()) == 1 && num_sides(t.top()) == 1) {
            break tile;
        }
       
    });

    loop {
        let Some(mut tile) = tiles.pop_back() else {
            break;
        };
        if grid.len() < 3 {
            eprintln!("less than 3");
            let neighbor_side = grid[grid.len()-1].right();


            if rotate_and_flip_until(&mut tile, |t| neighbor_side == t.left()) {
                grid.push(tile);
            } else {
                tiles.push_front(tile);
            }
        } else {
            eprintln!("found more than 3");
            let neighbor_side = grid[grid.len()-3].bottom();

            if rotate_and_flip_until(&mut tile, |t| neighbor_side == t.top()) {
                grid.push(tile);
            } else {
                tiles.push_front(tile);
            }
        }
    }
    dbg!(&grid);
    42
}
