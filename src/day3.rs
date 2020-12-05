use std::convert::TryFrom;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
pub enum MapSquare {
    Open,
    Tree,
}

impl TryFrom<u8> for MapSquare {
    type Error = u8;
    fn try_from(b: u8) -> Result<MapSquare, u8> {
        Ok(match b {
            b'.' => MapSquare::Open,
            b'#' => MapSquare::Tree,
            unknown => return Err(unknown),
        })
    }
}

#[aoc_generator(day3)]
pub fn d3_input(input: &[u8]) -> Vec<Vec<MapSquare>> {
    input
        .split(|&b| b == b'\n')
        .map(|line| {
            line.iter()
                .copied()
                .map(|b| MapSquare::try_from(b).unwrap())
                .collect()
        })
        .collect()
}

#[aoc(day3, part1)]
pub fn solve_d3_p1(input: &[Vec<MapSquare>]) -> usize {
    input
        .iter()
        .skip(1)
        .enumerate()
        .map(|(steps_taken, grid_line)| {
            let x_coord = (steps_taken + 1) * 3;
            match grid_line.iter().cycle().nth(x_coord).unwrap() {
                MapSquare::Open => 0,
                MapSquare::Tree => 1,
            }
        })
        .sum()
}

#[aoc(day3, part2)]
pub fn solve_d3_p2(input: &[Vec<MapSquare>]) -> usize {
    #[derive(Debug, Clone, Copy)]
    struct Step {
        x_step: usize,
        y_step: usize,
    }

    let steps = [
        Step {
            x_step: 1,
            y_step: 1,
        },
        Step {
            x_step: 3,
            y_step: 1,
        },
        Step {
            x_step: 5,
            y_step: 1,
        },
        Step {
            x_step: 7,
            y_step: 1,
        },
        Step {
            x_step: 1,
            y_step: 2,
        },
    ];

    steps
        .iter()
        .copied()
        .map(|Step { x_step, y_step }| -> usize {
            input
                .iter()
                .skip(y_step)
                .step_by(y_step)
                .enumerate()
                .map(|(num_steps_taken, grid_line)| {
                    let x_coord = (num_steps_taken + 1) * x_step;
                    match grid_line.iter().cycle().nth(x_coord).unwrap() {
                        MapSquare::Open => 0,
                        MapSquare::Tree => 1,
                    }
                })
                .sum()
        })
        .product()
}
