use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn d1_input(input: &str) -> Vec<u64> {
    let mut entries: Vec<u64> = input.split('\n').map(|x| x.parse().unwrap()).collect();
    entries.sort();
    entries
}

#[aoc(day1, part1)]
pub fn solve_d1_p1(input: &[u64]) -> u64 {
    for entry in input {
        let needed = 2020 - entry;
        if let Ok(idx) = input.binary_search(&needed) {
            return entry * input[idx];
        }
    }
    panic!("not found");
}

#[aoc(day1, part2)]
pub fn solve_d1_p2(input: &[u64]) -> Option<u64> {
    for i in 0..input.len() - 2 {
        let entry1 = input[i];
        for j in i + 1..input.len() - 1 {
            let entry2 = input[j];
            if entry1 + entry2 > 2020 {
                break;
            }
            for k in j + 1..input.len() {
                let entry3 = input[k];
                if entry1 + entry2 + entry3 == 2020 {
                    return Some(entry1 * entry2 * entry3);
                }
            }
        }
    }
    return None;
}
