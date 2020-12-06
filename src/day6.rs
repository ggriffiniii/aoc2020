use std::{iter::FromIterator, ops::BitAnd};

use aoc_runner_derive::aoc;

/// A set that can only contain b'a'..b'z';
struct AlphabetSet(u32);
impl AlphabetSet {
    pub fn new() -> AlphabetSet {
        AlphabetSet(0)
    }

    pub fn insert(&mut self, b: u8) {
        self.0 |= 1 << Self::char_to_bit_index(b);
    }

    pub fn len(&self) -> usize {
        self.0.count_ones() as usize
    }

    fn char_to_bit_index(b: u8) -> usize {
        debug_assert!(b >= b'a' && b <= b'z');
        (b as usize - 97) & 0x1f
    }
}

// Do a set intersection with another set.
impl std::ops::BitAndAssign for AlphabetSet {
    fn bitand_assign(&mut self, rhs: AlphabetSet) {
        self.0 &= rhs.0;
    }
}

impl FromIterator<u8> for AlphabetSet {
    fn from_iter<I>(iter: I) -> AlphabetSet
    where
        I: IntoIterator<Item = u8>,
    {
        let mut set = AlphabetSet::new();
        for entry in iter {
            set.insert(entry);
        }
        set
    }
}

#[aoc(day6, part1)]
pub fn solve_d6_p1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|record| {
            record
                .split('\n')
                .flat_map(|line| line.as_bytes())
                .copied()
                .collect::<AlphabetSet>()
                .len()
        })
        .sum()
}

#[aoc(day6, part2)]
pub fn solve_d6_p2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|record| {
            let mut line_iter = record
                .split('\n')
                .map(|line| line.as_bytes().iter().copied().collect::<AlphabetSet>());
            let mut group_set = line_iter.next().unwrap();
            for set in line_iter {
                group_set &= set;
            }
            group_set.len()
        })
        .sum()
}
