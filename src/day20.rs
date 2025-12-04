use crate::split_once;
use aoc_runner_derive::aoc;
use core::fmt;
use std::collections::HashMap;
use std::collections::VecDeque;

// Sea monster pattern
//                   #
// #    ##    ##    ###
//  #  #  #  #  #  #

const SEA_MONSTER_OFFSETS: &[(isize, isize)] = &[
    (0, 0),
    (1, -18),
    (1, -13),
    (1, -12),
    (1, -7),
    (1, -6),
    (1, -1),
    (1, 0),
    (1, 1),
    (2, -17),
    (2, -14),
    (2, -11),
    (2, -8),
    (2, -5),
    (2, -2),
];

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
        self.data
            .iter()
            .step_by(10)
            .cloned()
            .enumerate()
            .filter(|&(_, b)| b)
            .fold(0u16, |accum, (idx, _)| accum | (1 << idx))
    }

    fn right(&self) -> u16 {
        self.data
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
                update[flipped_row * 10 + coli] = self.data[rowi * 10 + coli];
            }
        }
        self.data = update;
    }

    fn rotate_90(&mut self) {
        let mut update = [false; 100];
        for rowi in 0..10 {
            let new_col = 9 - rowi;
            for coli in 0..10 {
                let new_row = coli;
                update[new_row * 10 + new_col] = self.data[rowi * 10 + coli];
            }
        }
        self.data = update;
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

    // Rotate and flip the tile in all permutations, stopping if matched ever returns true. Returned
    // true if a match was found, false if all permutations were exhausted without a match.
    fn rotate_and_flip_until<F>(&mut self, mut matched: F) -> bool
    where
        F: FnMut(&Tile) -> bool,
    {
        for _ in 0..4 {
            if matched(self) {
                return true;
            }
            self.rotate_90();
        }
        self.flip();
        for _ in 0..4 {
            if matched(self) {
                return true;
            }
            self.rotate_90();
        }
        false
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for rowi in 0..10 {
            for coli in 0..10 {
                let d = if self.data[rowi * 10 + coli] {
                    '#'
                } else {
                    '.'
                };
                write!(f, "{d}")?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

struct Image {
    // image is a square so row_len and col_len are the same.
    row_len: usize,
    data: Vec<bool>,
}

impl Image {
    fn new(grid: &[Tile]) -> Image {
        let tiles_per_row = grid.len().isqrt();
        // ensure we have a square
        assert_eq!(tiles_per_row * tiles_per_row, grid.len());
        let row_len = tiles_per_row * 8;

        let mut data = vec![false; 64 * grid.len()];
        let mut datai = 0;
        for gridrowi in 0..tiles_per_row {
            for tilerowi in 1..9 {
                for gridcoli in 0..tiles_per_row {
                    for tilecoli in 1..9 {
                        let grididx = gridrowi * tiles_per_row + gridcoli;
                        let tileidx = tilerowi * 10 + tilecoli;
                        data[datai] = grid[grididx].data[tileidx];
                        datai += 1;
                    }
                }
            }
        }
        Image { row_len, data }
    }

    fn flip(&mut self) {
        let mut update = self.data.clone();
        for rowi in 0..self.row_len {
            let flipped_row = self.row_len - 1 - rowi;
            for coli in 0..self.row_len {
                update[flipped_row * self.row_len + coli] = self.data[rowi * self.row_len + coli];
            }
        }
        self.data = update;
    }

    fn rotate_90(&mut self) {
        let mut update = self.data.clone();
        for rowi in 0..self.row_len {
            let new_col = self.row_len - 1 - rowi;
            for coli in 0..self.row_len {
                let new_row = coli;
                update[new_row * self.row_len + new_col] = self.data[rowi * self.row_len + coli];
            }
        }
        self.data = update;
    }

    // Rotate and flip the image in all permutations, stopping if matched ever returns true. Returned
    // true if a match was found, false if all permutations were exhausted without a match.
    fn rotate_and_flip_until<F>(&mut self, mut matched: F) -> bool
    where
        F: FnMut(&Image) -> bool,
    {
        for _ in 0..4 {
            if matched(self) {
                return true;
            }
            self.rotate_90();
        }
        self.flip();
        for _ in 0..4 {
            if matched(self) {
                return true;
            }
            self.rotate_90();
        }
        false
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for rowi in 0..self.row_len {
            for coli in 0..self.row_len {
                let d = if self.data[rowi * self.row_len + coli] {
                    '#'
                } else {
                    '.'
                };
                write!(f, "{d}")?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

#[aoc(day20, part1)]
fn solve_d20_p1(input: &str) -> usize {
    let tiles: Vec<_> = input
        .split("\n\n")
        .map(|i| Tile::parse(i).unwrap())
        .collect();
    let mut side_to_tile: HashMap<_, Vec<_>> = HashMap::new();
    for tile in &tiles {
        for side in tile.sides() {
            side_to_tile
                .entry(Tile::side_id(side))
                .or_default()
                .push(tile.id);
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

#[aoc(day20, part2)]
fn solve_d20_p2(input: &str) -> usize {
    //let input = EXAMPLE;
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
    let num_sides = |side| *side_occurrences.get(&Tile::side_id(side)).unwrap();

    let mut grid = Vec::new();
    let tiles_per_row = tiles.len().isqrt();
    // ensure we have a square.
    assert_eq!(tiles_per_row * tiles_per_row, tiles.len());

    grid.push(loop {
        let mut tile = tiles.pop_back().unwrap();
        let num_unique_sides = tile
            .sides()
            .into_iter()
            .filter(|&side| num_sides(side) == 1)
            .count();
        if num_unique_sides != 2 {
            tiles.push_front(tile);
            continue;
        }

        if tile.rotate_and_flip_until(|t| num_sides(t.left()) == 1 && num_sides(t.top()) == 1) {
            break tile;
        }
    });

    loop {
        let Some(mut tile) = tiles.pop_back() else {
            break;
        };
        if grid.len() < tiles_per_row {
            let neighbor_side = grid[grid.len() - 1].right();

            if tile.rotate_and_flip_until(|t| neighbor_side == t.left()) {
                grid.push(tile);
            } else {
                tiles.push_front(tile);
            }
        } else {
            let neighbor_side = grid[grid.len() - tiles_per_row].bottom();

            if tile.rotate_and_flip_until(|t| neighbor_side == t.top()) {
                grid.push(tile);
            } else {
                tiles.push_front(tile);
            }
        }
    }
    for g in &grid {
        eprintln!("{}", g.id);
    }
    let mut image = Image::new(&grid);
    eprintln!("{image}");
    let mut num_monsters = 0;
    let mut monster_locations = Vec::new();
    image.rotate_and_flip_until(|img| {
        let mut found = false;
        for rowi in 0..img.row_len as isize {
            for coli in 0..img.row_len as isize {
                let found_monster = SEA_MONSTER_OFFSETS.iter().all(|&(row_offset, col_offset)| {
                    if rowi + row_offset < 0
                        || rowi + row_offset >= img.row_len as isize
                        || coli + col_offset < 0
                        || coli + col_offset >= img.row_len as isize
                    {
                        return false;
                    }
                    let idx = (rowi + row_offset) * img.row_len as isize + (coli + col_offset);
                    img.data[idx as usize]
                });
                if found_monster {
                    monster_locations.extend(SEA_MONSTER_OFFSETS.iter().map(
                        |&(row_offset, col_offset)| {
                            ((rowi + row_offset) * img.row_len as isize + (coli + col_offset))
                                as usize
                        },
                    ));
                    num_monsters += 1;
                    found = true;
                }
            }
        }
        found
    });
    for idx in monster_locations {
        image.data[idx] = false;
    }
    image.data.iter().filter(|&&b| b).count()
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
