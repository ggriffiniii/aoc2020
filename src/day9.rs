use aoc_runner_derive::aoc;

use std::collections::HashSet;

struct Preamble {
    entries: [usize; 25],
    entries_start_idx: usize,
    set: HashSet<usize>,
}

impl Preamble {
    // Take the first 25 items from the iterator.
    fn new<I>(iter: &mut I) -> Option<Self>
    where
        I: Iterator<Item = usize>,
    {
        let mut entries = [0; 25];
        let mut set = HashSet::new();
        let mut count = 0;
        for (idx, entry) in iter.enumerate().take(25) {
            count += 1;
            entries[idx] = entry;
            set.insert(entry);
        }
        if count != 25 {
            return None;
        }

        Some(Preamble {
            entries,
            entries_start_idx: 0,
            set,
        })
    }

    // Insert a new entry into the preamble letting the oldest one roll off.
    fn insert(&mut self, entry: usize) {
        self.set.remove(&self.entries[self.entries_start_idx]);
        self.set.insert(entry);
        self.entries[self.entries_start_idx] = entry;
        self.entries_start_idx = (self.entries_start_idx + 1) % 25;
    }

    fn find_sum_pair(&self, total: usize) -> Option<(usize, usize)> {
        for idx in 0..25 {
            let entry = self.entries[(self.entries_start_idx + idx) % 25];
            let needed = total - entry;
            if self.set.contains(&needed) {
                return Some((entry, needed));
            }
        }
        None
    }
}

#[aoc(day9, part1)]
pub fn solve_d9_p1(input: &str) -> usize {
    let mut iter = input.split('\n').map(|x| -> usize { x.parse().unwrap() });
    let mut preamble = Preamble::new(&mut iter).unwrap();
    for entry in iter {
        if preamble.find_sum_pair(entry).is_none() {
            return entry;
        }
        preamble.insert(entry);
    }
    panic!("not found");
}

#[aoc(day9, part2)]
pub fn solve_d9_p2(input: &str) -> usize {
    const EXPECTED_SUM: usize = 36845998;
    let entries: Vec<usize> = input
        .split('\n')
        .map(|x| -> usize { x.parse().unwrap() })
        .collect();

    let mut sum = 0;
    let mut start_idx = 0;
    let mut end_idx = 0;
    loop {
        if sum < EXPECTED_SUM {
            sum += entries[end_idx];
            end_idx += 1;
        } else if sum > EXPECTED_SUM {
            sum -= entries[start_idx];
            start_idx += 1;
        } else {
            let (min, max) = &entries[start_idx..end_idx]
                .iter()
                .copied()
                .fold((usize::MAX, 0), |(min, max), entry| {
                    (std::cmp::min(min, entry), std::cmp::max(max, entry))
                });
            return min + max;
        }
    }
}
