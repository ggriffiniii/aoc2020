use aoc_runner_derive::aoc;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Pos(isize, isize);
impl Pos {
    fn step(self, dir: Direction) -> Self {
        match dir {
            Direction::E => Pos(self.0 + 2, self.1),
            Direction::SE => Pos(self.0 + 1, self.1 -1),
            Direction::SW => Pos(self.0 -1, self.1 -1),
            Direction::W => Pos(self.0 -2, self.1),
            Direction::NE => Pos(self.0 + 1, self.1 + 1),
            Direction::NW => Pos(self.0 -1, self.1 + 1),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}

impl Direction {
    fn parse(input: &[u8]) -> Option<(&[u8], Direction)> {
        match input {
            &[b's', b'e', ..] => Some((&input[2..], Direction::SE)),
            &[b's', b'w', ..] => Some((&input[2..], Direction::SW)),
            &[b'n', b'e', ..] => Some((&input[2..], Direction::NE)),
            &[b'n', b'w', ..] => Some((&input[2..], Direction::NW)),
            &[b'e', ..] => Some((&input[1..], Direction::E)),
            &[b'w', ..] => Some((&input[1..], Direction::W)),
            _ => None,
        }
    }

    fn iter(input: &[u8]) -> impl Iterator<Item = Direction> + '_ {
        struct Iter<'a>(&'a [u8]);
        impl<'a> Iterator for Iter<'a> {
            type Item = Direction;
            fn next(&mut self) -> Option<Self::Item> {
                match Direction::parse(self.0) {
                    None => None,
                    Some((rem, dir)) => {
                        self.0 = rem;
                        Some(dir)
                    }
                }
            }
        }
        Iter(input)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Black,
    White,
}
impl Tile {
    fn flip(&mut self) {
        *self = match *self {
            Tile::Black => Tile::White,
            Tile::White => Tile::Black,
        };
    }
}

#[aoc(day24, part1)]
fn solve_d24_p1(input: &str) -> usize {
    let mut tiles: HashMap<_, usize> = HashMap::new();
    for pos in input.split('\n').map(|line| {
        Direction::iter(line.as_bytes()).fold(Pos(0, 0), |pos, dir| {
            pos.step(dir)
        })
    }) {
        *tiles.entry(pos).or_default() += 1;
    }

    tiles
        .values()
        .filter(|&times_flipped| times_flipped % 2 == 1)
        .count()
}

#[aoc(day24, part2)]
fn solve_d24_p2(input: &str) -> usize {
    let mut floor: HashMap<_, Tile> = HashMap::new();
    for pos in input.split('\n').map(|line| {
        Direction::iter(line.as_bytes()).fold(Pos(0, 0), |pos, dir| {
            pos.step(dir)
        })
    }) {
        floor.entry(pos).or_insert(Tile::White).flip()
    }

    for _ in 0..100 {
        let mut tiles_visited = HashMap::new();
        for &pos in floor.keys() {
            run(&floor, &mut tiles_visited, pos);
        }
        for pos in tiles_visited
            .into_iter()
            .filter_map(|(k, should_flip)| if should_flip { Some(k) } else { None })
        {
            floor.entry(pos).or_insert(Tile::White).flip();
        }
    }

    floor.values().filter(|&&tile| tile == Tile::Black).count()
}

fn run(
    floor: &HashMap<Pos, Tile>,
    tiles_visited: &mut HashMap<Pos, bool>,
    pos: Pos,
) {
    if tiles_visited.contains_key(&pos) {
        return;
    }
    let n = neighbors(floor, pos);
    let black_tiles = n
        .iter()
        .copied()
        .filter(|&tile| tile == Tile::Black)
        .count();
    let tile = floor.get(&pos).copied().unwrap_or(Tile::White);
    if (tile == Tile::Black && (black_tiles == 0 || black_tiles > 2))
        || (tile == Tile::White && black_tiles == 2)
    {
        tiles_visited.insert(pos, true);
    } else {
        tiles_visited.insert(pos, false);
    }
    if black_tiles > 0 {
        run(floor, tiles_visited, pos.step(Direction::E));
        run(floor, tiles_visited, pos.step(Direction::SE));
        run(floor, tiles_visited, pos.step(Direction::SW));
        run(floor, tiles_visited, pos.step(Direction::W));
        run(floor, tiles_visited, pos.step(Direction::NE));
        run(floor, tiles_visited, pos.step(Direction::NW));
    }
}

fn neighbors(floor: &HashMap<Pos, Tile>, pos: Pos) -> [Tile; 6] {
    [
        floor.get(&pos.step(Direction::E)).copied().unwrap_or(Tile::White),
        floor.get(&pos.step(Direction::SE)).copied().unwrap_or(Tile::White),
        floor.get(&pos.step(Direction::SW)).copied().unwrap_or(Tile::White),
        floor.get(&pos.step(Direction::W)).copied().unwrap_or(Tile::White),
        floor.get(&pos.step(Direction::NE)).copied().unwrap_or(Tile::White),
        floor.get(&pos.step(Direction::NW)).copied().unwrap_or(Tile::White),
    ]
}
