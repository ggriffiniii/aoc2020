use aoc_runner_derive::aoc;
use std::collections::HashMap;

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

    fn step(self) -> (isize, isize) {
        match self {
            Direction::E => (2, 0),
            Direction::SE => (1, -1),
            Direction::SW => (-1, -1),
            Direction::W => (-2, 0),
            Direction::NE => (1, 1),
            Direction::NW => (-1, 1),
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

#[aoc(day24, part1)]
fn solve_d24_p1(input: &str) -> usize {
    let mut tiles: HashMap<_, usize> = HashMap::new();
    for pos in input.split('\n').map(|line| {
        Direction::iter(line.as_bytes()).fold((0, 0), |pos, dir| {
            let (x_step, y_step) = dir.step();
            (pos.0 + x_step, pos.1 + y_step)
        })
    }) {
        *tiles.entry(pos).or_default() += 1;
    }

    tiles
        .values()
        .filter(|&times_flipped| times_flipped % 2 == 1)
        .count()
}
