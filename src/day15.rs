use aoc_runner_derive::aoc;

use std::collections::{hash_map::Entry, HashMap};

fn spoken_word_n(mut spoken: HashMap<usize, usize>, mut last_spoken: usize, n: usize) -> usize {
    for turn in spoken.len() + 1..=n {
        //dbg!(turn, last_spoken);
        last_spoken = match spoken.entry(last_spoken) {
            Entry::Occupied(mut occupied) => {
                let difference = turn - 1 - *occupied.get();
                occupied.insert(turn - 1);
                difference
            }
            Entry::Vacant(vacant) => {
                vacant.insert(turn - 1);
                0
            }
        }
    }
    last_spoken
}

#[aoc(day15, part1)]
fn solve_d15_p1(input: &str) -> usize {
    let mut last_spoken = None;
    let spoken: HashMap<usize, usize> = input
        .split(',')
        .enumerate()
        .map(|(idx, spoken)| {
            let spoken = spoken.parse().unwrap();
            last_spoken = Some(spoken);
            (spoken, idx + 1)
        })
        .collect();
    spoken_word_n(spoken, last_spoken.unwrap(), 2020)
}

#[aoc(day15, part2)]
fn solve_d15_p2(input: &str) -> usize {
    let mut last_spoken = None;
    let spoken: HashMap<usize, usize> = input
        .split(',')
        .enumerate()
        .map(|(idx, spoken)| {
            let spoken = spoken.parse().unwrap();
            last_spoken = Some(spoken);
            (spoken, idx + 1)
        })
        .collect();
    spoken_word_n(spoken, last_spoken.unwrap(), 30_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve_d15_p1("1,3,2"), 1);
    }
}
